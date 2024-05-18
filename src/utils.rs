use sha3::{self, Digest as SHA3Digest};
use sha2::{self};
use std;
use num_integer;
use gethostname::gethostname;

pub enum SHAs {
    /// Improved security and randomness
    SHA3_512,
    /// Less secure, beware!
    SHA2_512
}

pub fn create_counter(mut count: isize) -> CounterFunctionType {
    return Box::new(
        move || {
            count += 1;
            return count;
        }
    )
}

pub type RandomFunctionType = Box<dyn FnMut() -> f64>;
pub type CounterFunctionType = Box<dyn FnMut() -> isize>;
pub type FingerPrintFunctionType = Box<dyn Fn(&mut RandomFunctionType, Option<String>, &SHAs) -> String>;

const BIG_LENGTH: usize = 32;
const SHENANIGANS: [char; 36] = [
    'a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z','0','1','2','3','4','5','6','7','8','9'
];
const SHENANIGANS_LENGTH: u128 = SHENANIGANS.len() as u128;
const SHENANIGANS_LOWERCASE: [char; 26] = [
    'a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z'
];

pub fn create_fingerprint(random_number_generator: &mut RandomFunctionType, fingerprint_data: Option<String>, sha_algo: SHAs) -> String {
    let new_fingerprint_data: String;
    if fingerprint_data.is_none() {
        let process_id = std::process::id();
        let host_name = gethostname();
        let env_vars = std::env::vars().map(|(k, _)| {
            return k
        }).collect::<Vec<String>>().join("");
        new_fingerprint_data = process_id.to_string() + &host_name.to_string_lossy() + env_vars.as_str();
    } else {
        new_fingerprint_data = fingerprint_data.unwrap();
    }
    let fingerprint = new_fingerprint_data + create_entropy(random_number_generator, Some(BIG_LENGTH)).unwrap().as_str();
    return create_hash(Some(fingerprint), &sha_algo)[0..BIG_LENGTH].to_string();
}

pub fn create_hash(data: Option<String>, sha_algo: &SHAs) -> String {
    let actual_data = data.unwrap_or("".to_string());
    let hashed_data = match sha_algo {
        SHAs::SHA2_512 => {
            let mut hasher = sha2::Sha512::new();
            hasher.update(actual_data);
            hasher.finalize()
        },
        SHAs::SHA3_512 => {
            let mut hasher = sha3::Sha3_512::new();
            hasher.update(actual_data);
            hasher.finalize()
        }
    };
    let hexed = hex::encode(hashed_data);
    let hashed_int = u128::from_str_radix(&hexed, 16).unwrap();
    println!("{}",base36_encode(hashed_int as u128));
    return base36_encode(hashed_int as u128);
}

pub fn create_entropy(random_number_generator: &mut RandomFunctionType, length: Option<usize>) -> Result<String, ()> {
    let actual_length = length.unwrap_or(4);
    if actual_length < 1 {
        return Err(())
    }
    let mut entropy = String::new();
    while entropy.len() < actual_length {
        entropy += base36_encode((random_number_generator() * 36.0).floor() as u128).as_str();
    }
    return Ok(entropy);
}

pub fn base36_encode(mut number: u128) -> String {
    let mut encoded_string = String::new();
    let mut modular: u128;
    while number != 0 {
        (number, modular) = num_integer::div_rem(number, SHENANIGANS_LENGTH);
        encoded_string = SHENANIGANS[modular as usize].to_string() + &encoded_string;
    }
    if encoded_string.len() == 0 {
        println!("empty {}", number);
        return "0".to_string();
    }
    return encoded_string;
}

pub fn create_letter(random_number_generator: &mut RandomFunctionType) -> char {
    return SHENANIGANS_LOWERCASE[(random_number_generator() * (SHENANIGANS_LOWERCASE.len() as f64)) as usize]
}