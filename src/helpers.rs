use core::iter::Sum;

use num_traits::{FromPrimitive, Num};

/// Trait for a number-like type that we can calculate statistics on.
pub trait NumExt: Num + FromPrimitive + Copy + Sum {}
impl<T> NumExt for T where T: Num + FromPrimitive + Copy + Sum {}

/// Trait for types that support returning whether one is greater than the otehr
pub trait MinMax {
    /// Return the smaller of `self` and `other`
    fn min(self, other: Self) -> Self;

    /// Return the larger of `self` and `other`
    fn max(self, other: Self) -> Self;
}

macro_rules! impl_min_max_using_ord {
    ($($type: ty)*) => {
        $(impl MinMax for $type {
            fn min(self, other: Self) -> Self {
                <Self as Ord>::min(self, other)
            }

            fn max(self, other: Self) -> Self {
                <Self as Ord>::max(self, other)
            }
        })*
    };
}

macro_rules! impl_min_max_using_assoc_func {
    ($($type: ty)*) => {
        $(impl MinMax for $type {
            fn min(self, other: Self) -> Self {
                <$type>::min(self, other)
            }

            fn max(self, other: Self) -> Self {
                <$type>::max(self, other)
            }
        })*
    };
}

impl_min_max_using_ord!(i8 i16 i32 i64 i128);
impl_min_max_using_ord!(u8 u16 u32 u64 u128);
impl_min_max_using_assoc_func!(f32 f64);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_f32() {
        assert_eq!(<f32 as MinMax>::min(0.0, 1.0), 0.0);
    }

    #[test]
    fn test_max_f32() {
        assert_eq!(<f32 as MinMax>::max(0.0, 1.0), 1.0);
    }

    #[test]
    fn test_min_f32_nan() {
        assert_eq!(<f32 as MinMax>::min(1.0, f32::NAN), 1.0);
    }

    #[test]
    fn test_max_f32_nan() {
        assert_eq!(<f32 as MinMax>::max(1.0, f32::NAN), 1.0);
    }

    #[test]
    fn test_min_i32() {
        assert_eq!(<i32 as MinMax>::min(0, 1), 0);
    }

    #[test]
    fn test_max_i32() {
        assert_eq!(<i32 as MinMax>::max(0, 1), 1);
    }
}
