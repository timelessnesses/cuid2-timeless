use gethostname::gethostname;
use num_bigint::{self, BigUint};
use num_integer;
use std;
#[cfg(feature = "regex")]
use regex;

/// Default counter function
pub fn create_counter(mut count: isize) -> CounterFunctionType {
    return Box::new(move || {
        count += 1;
        return count;
    });
}

/// Random function type
pub type RandomFunctionType = Box<dyn FnMut() -> f64>;
/// Create counter function type
pub type CreateCounterFunctionType = Box<dyn Fn(isize) -> CounterFunctionType>;
/// Counter function type
pub type CounterFunctionType = Box<dyn FnMut() -> isize>;
/// Fingerprint function type
pub type FingerPrintFunctionType =
    Box<dyn Fn(&mut RandomFunctionType, Option<String>) -> String>;

/// Stupidities
const BIG_LENGTH: usize = 32;
const SHENANIGANS: [char; 36] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
];
const SHENANIGANS_LENGTH: u128 = SHENANIGANS.len() as u128;
const SHENANIGANS_LOWERCASE: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

/// Creates a machine-specific fingerprint
pub fn create_fingerprint(
    random_number_generator: &mut RandomFunctionType,
    fingerprint_data: Option<String>,
) -> String {
    let new_fingerprint_data: String;
    if fingerprint_data.is_none() {
        let process_id = std::process::id();
        let host_name = gethostname();
        let env_vars = std::env::vars()
            .map(|(k, _)| return k)
            .collect::<Vec<String>>()
            .join("");
        new_fingerprint_data =
            process_id.to_string() + &host_name.to_string_lossy() + env_vars.as_str();
    } else {
        new_fingerprint_data = fingerprint_data.unwrap();
    }
    let fingerprint = new_fingerprint_data
        + create_entropy(random_number_generator, Some(BIG_LENGTH))
            .unwrap()
            .as_str();
    return create_hash(Some(fingerprint))[0..BIG_LENGTH].to_string();
}

/// Creates hash based on data and hash it with SHA based on feature flag you specified
pub fn create_hash(data: Option<String>) -> String {
    let actual_data = data.unwrap_or("".to_string());
    
    #[cfg(feature="sha2")]
    use sha2::{self, Digest};
    #[cfg(feature="sha2")]
    let mut hasher = sha2::Sha512::new();
    #[cfg(feature="sha3")]
    use sha3::{self, Digest};
    #[cfg(feature="sha3")]
    let mut hasher = sha3::Sha3_512::new();

    hasher.update(actual_data);
    let hashed_value = hasher.finalize();

    let hashed_int = num_bigint::BigUint::from_bytes_be(&hashed_value);
    return base36_encode(hashed_int);
}

/// Creates entropy by length
pub fn create_entropy(
    random_number_generator: &mut RandomFunctionType,
    length: Option<usize>,
) -> Result<String, ()> {
    let actual_length = length.unwrap_or(4);
    if actual_length < 1 {
        return Err(());
    }
    let mut entropy = String::new();
    while entropy.len() < actual_length {
        entropy += base36_encode(num_bigint::BigUint::from(
            (random_number_generator() * 36.0).floor() as u128,
        ))
        .as_str();
    }
    return Ok(entropy);
}

/// Base36 Encoder
pub fn base36_encode(mut number: num_bigint::BigUint) -> String {
    let mut encoded_string = String::new();
    let mut modular: num_bigint::BigUint;
    while number != BigUint::from(0 as usize) {
        (number, modular) = num_integer::div_rem(number, SHENANIGANS_LENGTH.into());
        encoded_string = SHENANIGANS[modular.to_string().parse::<usize>().unwrap()].to_string()
            + &encoded_string;
    }
    if encoded_string.len() == 0 {
        return "0".to_string();
    }
    return encoded_string;
}

/// Randomly selects a letter from [`RandomFunctionType`] value
pub fn create_letter(random_number_generator: &mut RandomFunctionType) -> char {
    return SHENANIGANS_LOWERCASE
        [(random_number_generator() * (SHENANIGANS_LOWERCASE.len() as f64)) as usize];
}

#[cfg(feature = "regex")]
/// Checks if this is valid [`crate::Cuid`]
pub fn is_cuid(id: String, min_length: Option<usize>, max_length: Option<usize>) -> bool {
    let length = id.len();
    let re = r"^[0-9a-z]+$";
    if length >= min_length.unwrap_or(2) && length <= max_length.unwrap_or(crate::generator::MAXIMUM_LENGTH) {
        return regex::Regex::new(re).unwrap().is_match(&id)
    }
    return false;
}