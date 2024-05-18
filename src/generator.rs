use rand::{self, Rng};

use crate::{
    errors,
    utils::{self, SHAs},
};

pub const INITIAL_COUNT_MAX: usize = 476782367;
pub const DEFAULT_LENGTH: usize = 24;
pub const MAXIMUM_LENGTH: usize = 98;

pub struct Cuid {
    random: utils::RandomFunctionType,
    counter: utils::CounterFunctionType,
    length: usize,
    fingerprint: String,
    sha_algo: SHAs,
}

impl Default for Cuid {
    fn default() -> Self {
        let mut randomity = rand::thread_rng();
        let randomed: f64 = randomity.gen();
        let mut wrapper_rand: Box<dyn FnMut() -> f64> = Box::new(move || randomity.gen());
        Cuid {
            fingerprint: utils::create_fingerprint(&mut wrapper_rand, None, utils::SHAs::SHA3_512),
            random: wrapper_rand,
            counter: Box::new(utils::create_counter(
                (randomed * INITIAL_COUNT_MAX as f64) as isize,
            )),
            length: DEFAULT_LENGTH,
            sha_algo: utils::SHAs::SHA3_512,
        }
    }
}

impl Cuid {
    #[inline]
    pub fn new(
        mut random: utils::RandomFunctionType,
        counter: utils::CounterFunctionType,
        length: usize,
        fingerprint: utils::FingerPrintFunctionType,
        sha_algo: SHAs,
    ) -> Self {
        Cuid {
            fingerprint: fingerprint(&mut random, None, &sha_algo),
            random,
            counter,
            length,
            sha_algo,
        }
    }
    #[inline]
    pub fn generate(&mut self, length: Option<usize>) -> Result<String, errors::Errors> {
        let actual_length = length.unwrap_or(self.length);
        if actual_length > MAXIMUM_LENGTH {
            return Err(errors::Errors::ExceededMaximumLengthGenerateCuidError);
        }

        let first_letter = utils::create_letter(&mut self.random);

        let base36_time = utils::base36_encode(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
                .into(),
        );
        let base36_count = utils::base36_encode(((self.counter)() as usize).into());

        let salt = match utils::create_entropy(&mut self.random, Some(actual_length))
            .map_err(|_| errors::Errors::LessThanOneEntropyError)
        {
            Ok(s) => s,
            Err(e) => {
                return Err(e);
            }
        };
        let hash_input = base36_time + &salt + &base36_count + &self.fingerprint;

        return Ok(first_letter.to_string()
            + &utils::create_hash(Some(hash_input), &self.sha_algo)[1..actual_length]);
    }
}

pub fn cuid_wrapper() -> Box<dyn FnMut() -> Result<String, errors::Errors>> {
    let mut cuid = Cuid::default();
    return Box::new(move || cuid.generate(None));
}
