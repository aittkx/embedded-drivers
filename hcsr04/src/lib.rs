#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(not(test), no_std)]
#![deny(unsafe_code)]
#![warn(missing_docs)]
#![cfg_attr(not(doctest), doc = include_str!("../README.md"))]

pub mod error;
pub mod hcsr04;
pub mod temperature;

// Re-exports for convenience
pub use error::Error;
pub use hcsr04::Hcsr04;
pub use temperature::{NoTemperatureCompensation, TemperatureProvider};

// Additional Feature specific modules and re-exports

/// This alias simplifies function signatures by defaulting the error type
/// to the crate's custom [`Error`] enum.
pub(crate) type Result<T> = core::result::Result<T, crate::error::Error>;
