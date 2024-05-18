use std::{error::Error, fmt::Display};

#[derive(Clone, Copy, Debug)]
pub enum Errors {
    /// [`crate::utils::create_entropy`] error for length less than 1
    LessThanOneEntropyError,
    /// [`crate::utils::base36_encode`] error for integer is negative (impossible)
    CannotEncodeNegativeIntegersBase36EncodeError,
    /// [`crate::Cuid::generate`] error for setting length longer than [`crate::generator::INITIAL_COUNT_MAX`]
    ExceededMaximumLengthGenerateCuidError,
}

impl Error for Errors {}
impl Display for Errors {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}
