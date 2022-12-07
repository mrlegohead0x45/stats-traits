//! Library for calculating statistics on collections of numbers.

#![warn(missing_docs)]
#![no_std]

use core::iter::Sum;

use num_traits::{FromPrimitive, Num};

mod error;
mod freq;
mod stats;

/// Trait for a number-like type that we can calculate statistics on.
pub trait NumExt: Num + FromPrimitive + Copy + Sum {}
impl<T> NumExt for T where T: Num + FromPrimitive + Copy + Sum {}

pub use crate::error::StatsError;
pub use crate::freq::FrequencyStats;
pub use crate::stats::Stats;
pub use crate::types::Result;

/// Module with type aliases
pub mod types {
    /// Type alias for a [`Result`] with the error type set to [`StatsError`]
    pub type Result<T> = core::result::Result<T, crate::StatsError>;
}
