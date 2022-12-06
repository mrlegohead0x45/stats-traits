//! Library for calculating statistics on collections of numbers.

#![warn(missing_docs)]
#![no_std]

use core::iter::Sum;

use num_traits::{FromPrimitive, Num};

mod stats;

/// Trait for a number-like type that we can calculate statistics on.
pub trait NumExt: Num + FromPrimitive + Copy + Sum {}
impl<T> NumExt for T where T: Num + FromPrimitive + Copy + Sum {}

pub use crate::stats::Stats;
