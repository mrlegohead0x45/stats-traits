//! Library for calculating statistics on collections of numbers.

#![warn(missing_docs)]
#![warn(clippy::cargo)]
#![no_std]

mod error;
mod freq;
mod helpers;
mod stats;

pub use crate::error::StatsError;
pub use crate::freq::FrequencyStats;
pub use crate::stats::Stats;
pub use crate::types::Result;

/// Module with type aliases
pub mod types {
    /// Type alias for a [`Result`] with the error type set to [`StatsError`](crate::StatsError)
    pub type Result<T> = core::result::Result<T, crate::StatsError>;
}
