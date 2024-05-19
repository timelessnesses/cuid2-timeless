//! # cuid2-timeless
//! 
//! An untested, somewhat stable CUID2 generator library for rust. Built for learning more rust and I don't know how to test it.
//! 
//! ## Usage
//! 
//! Either use the `cuid2_timeless::cuid_wrapper()` and call it again to get ID or get some customization with `cuid2_timeless::Cuid`
//! 
//! ``` rust
//! use cuid2_timeless;
//! 
//! let mut cuid = cuid2_timeless::cuid_wrapper();
//! println!("{}", cuid().unwrap());
//! ```
//! 
//! or some customization
//! 
//! ```rust
//! use cuid2_timeless;
//! use rand::{self, Rng};
//! 
//! let mut cuid = cuid2_timeless::Cuid::new(
//!     Box::new( // A randomizer function
//!         || {
//!             rand::thread_rng().gen()
//!         }
//!     ),
//!     Box::new( // Your very own counter function
//!         |mut i| {
//!             Box::new(move || {
//!                 i += 1;
//!                 i
//!             })
//!         }
//!     ),
//!     24, // Length
//!     Box::new(cuid2_timeless::utils::create_fingerprint) // And a fingerprint creator function (im too lazy to implement)
//! );
//! cuid.generate(None).unwrap();
//! ```
//! 
//! or verify if it actually is `CUID`
//! 
//! ```rust
//! use cuid2_timeless;
//! 
//! println!("{}", cuid2_timeless::is_cuid("f9ovgvsnly7h6khwt4nx08kom".to_string(), None, None));
//! ```
//! ## Features
//! 
//! - `regex` is a feature for either remove or add [`regex`] crate to the module. Turning this off will remove the [`is_cuid`] function (enabled by default)
//! - `random` is a feature for either remove or add [`rand`] crate to the module. Turning this off will remove [`Default`] trait from [`Cuid`] (enabled by default)
//! - `sha2` is a feature for either remove or add `sha2` crate to the module. Turning this off will remove SHA2 hashing algorithm, but turning this feature on will uses `sha2` for hashing (cannot enabled with `sha3` present, compiling will error if `sha2` and `sha3` is enabled, not enabled by default)
//! - `sha3` is a feature for either remove or add `sha3` crate to the module. Turning this off will remove SHA3 hashing algorithm, but turning this feature on will uses `sha3` for hashing (cannot enabled with `sha2` present, compiling will error if `sha2` and `sha3` is enabled, enabled by default)

#![warn(missing_docs)]

/// Errors module that holds a bunch of [`crate::errors::Errors`] values and implementations
pub mod errors;
/// Generator module that holds [`crate::Cuid`]
pub mod generator;

/// Utilities
pub mod utils;

pub use generator::Cuid;

#[cfg(feature = "random")]
pub use generator::cuid_wrapper;

#[cfg(feature = "regex")]
pub use utils::is_cuid;
#[cfg(all(test, feature="tests"))]
/// it's tests, what do you expect
pub mod tests;
