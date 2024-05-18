use std::{error::Error, fmt::Display};

#[derive(Clone, Copy, Debug)]
pub enum Errors {
    LessThanOneEntropyError,
    CannotEncodeNegativeIntegersBase36EncodeError,
    ExceededMaximumLengthGenerateCuidError
}

impl Error for Errors {}
impl Display for Errors {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}