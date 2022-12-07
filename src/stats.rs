use num_traits::{FromPrimitive, ToPrimitive};

use crate::error::DataType;
use crate::NumExt;
use crate::Result;
use crate::StatsError;

/// A trait to be implemented for collection-like types
/// that provides statistical methods. Requires that the
/// type it is implemented on can be converted into an
/// [`Iterator`].
pub trait Stats: IntoIterator + Clone
where
    Self::Item: NumExt,
{
    /// Calculate the sum of all the items in the collection
    ///
    /// # Example
    /// ```
    /// use stats_traits::Stats;
    /// assert_eq!(vec![1, 2, 3].sum(), 6);
    /// ```
    /// ```
    /// use stats_traits::Stats;
    /// assert_eq!(vec![1.0, 2.0, 3.0].sum(), 6.0);
    /// ```
    #[inline]
    fn sum(&self) -> Self::Item {
        self.clone().into_iter().sum()
    }

    /// Count the items in the collection
    ///
    /// # Example
    /// ```
    /// use stats_traits::Stats;
    /// assert_eq!(vec![1, 2, 3].count(), 3);
    /// ```
    #[inline]
    fn count(&self) -> usize {
        self.clone().into_iter().count()
    }

    /// Count the items in the collection, returning
    /// `Err(StatsError::EmptyCollection)` if it's empty
    fn non_zero_count(&self) -> Result<usize> {
        let count = self.count();
        if count == 0 {
            Err(StatsError::EmptyCollection)
        } else {
            Ok(count)
        }
    }

    /// Count the items in the collection and convert the result
    /// to [`Self::Item`]. Return an error
    /// under the same conditions as [`Stats::non_zero_count`],
    /// or if the length could not be converted to [`Self::Item`]
    ///
    /// [`Self::Item`]: IntoIterator::Item
    fn non_zero_count_as_item(&self) -> Result<Self::Item> {
        match Self::Item::from_usize(self.non_zero_count()?) {
            Some(count) => Ok(count),
            None => Err(StatsError::CouldNotConvert {
                from: DataType::Usize,
                to: DataType::Item,
            }),
        }
    }

    /// Find the mean of the collection
    ///
    /// # Examples
    /// ```
    /// use stats_traits::Stats;
    /// assert_eq!(vec![1, 2, 3].mean(), Ok(2));
    /// ```
    /// ```
    /// use stats_traits::Stats;
    /// let v: Vec<f64> = vec![1.0, 2.0, 3.0];
    /// assert_eq!(v.mean(), Ok(2.0));
    /// ```
    /// Watch out for integer division!
    /// ```
    /// use stats_traits::Stats;
    /// assert_eq!(vec![1, 2, 3, 4].mean(), Ok(2));
    /// ```
    ///
    /// # Errors
    /// Returns an error if the collection is empty (has a length of 0).
    /// Will also return an error if the length of the collection is too large
    /// to fit in [`Self::Item`](IntoIterator::Item).
    fn mean(&self) -> Result<Self::Item> {
        Ok(self.sum() / self.non_zero_count_as_item()?)
    }

    /// Find the variance of the collection.
    /// The variance is the sum of the squared differences between each item
    /// and the mean, divided by the number of items in the collection.
    /// The variance is a measure of how spread out the items are.
    ///
    /// # Examples
    /// ```
    /// use stats_traits::Stats;
    /// assert_eq!(vec![1.0, 2.0, 3.0].variance(), Ok(2.0 / 3.0));
    /// ```
    ///
    /// # Errors
    /// Errors under the same conditions as [`Stats::mean`] and
    /// [`Stats::non_zero_count_as_item`]
    ///
    /// [Wikipedia](<https://en.wikipedia.org/wiki/Variance>)
    fn variance(&self) -> Result<Self::Item> {
        let mean = self.mean()?;
        Ok(self
            .clone()
            .into_iter()
            .map(|x| (x - mean) * (x - mean))
            .sum::<Self::Item>()
            / self.non_zero_count_as_item()?)
    }

    /// Find the standard deviation of the collection.
    /// The standard deviation is the square root of the variance.
    /// The standard deviation is a measure of how spread out the items are.
    /// It is the square root of the average of the squared differences from the mean.
    ///
    /// # Examples
    /// ```
    /// use stats_traits::Stats;
    /// use approx::assert_relative_eq;
    /// assert_relative_eq!(vec![1.0, 2.0, 3.0].std_dev().unwrap(), 2.0_f64.sqrt() / 3.0_f64.sqrt());
    /// ```
    ///
    /// # Errors
    /// Returns an error under the same conditions as [`Stats::variance`]
    fn std_dev(&self) -> Result<Self::Item>
    where
        Self::Item: ToPrimitive,
    {
        Self::Item::from_f64(match self.variance()?.to_f64() {
            Some(x) => x.sqrt(),
            None => {
                return Err(StatsError::CouldNotConvert {
                    from: DataType::Item,
                    to: DataType::F64,
                })
            }
        })
        .ok_or(StatsError::CouldNotConvert {
            from: DataType::F64,
            to: DataType::Item,
        })
    }
}

/// Blanket implementation for all types that implement [`IntoIterator`] and [`Copy`].
/// This allows us to use the methods on any type that implements those traits.
/// For example, we can use the methods on `Vec` and `&[i32]`.
impl<T> Stats for T
where
    T: IntoIterator + Clone,
    T::Item: NumExt,
{
}

#[cfg(test)]
mod tests {
    use super::*;

    extern crate std;
    use std::prelude::rust_2021::*;
    use std::vec;

    use approx::assert_relative_eq;

    #[test]
    fn test_sum_vec() {
        let v = vec![1, 2, 3];
        assert_eq!(v.sum(), 6);

        let v = vec![1.0, 2.0, 3.0];
        assert_eq!(v.sum(), 6.0);
    }

    #[test]
    fn test_count_vec() {
        let v = vec![1, 2, 3];
        assert_eq!(v.count(), 3);
    }

    #[test]
    fn test_mean_vec() {
        let v = vec![1, 2, 3];
        assert_eq!(v.mean(), Ok(2));

        let v: Vec<f64> = vec![1.0, 2.0, 3.0];
        assert_eq!(v.mean(), Ok(2.0));

        let v = vec![1, 2, 3, 4];
        assert_eq!(v.mean(), Ok(2));
    }

    #[test]
    fn test_variance_vec() {
        let v = vec![1.0, 2.0, 3.0];
        assert_eq!(v.variance(), Ok(2.0 / 3.0));
    }

    #[test]
    fn test_std_dev_vec() {
        let v = vec![1.0, 2.0, 3.0];
        assert_relative_eq!(v.std_dev().unwrap(), 2.0_f64.sqrt() / 3.0_f64.sqrt());
    }

    #[test]
    fn test_non_zero_count_fail() {
        assert_eq!(
            Vec::<i32>::new().non_zero_count(),
            Err(StatsError::EmptyCollection)
        )
    }

    #[test]
    fn test_non_zero_count_as_item_fail() {
        assert_eq!(
            Vec::<i8>::from_iter(std::iter::repeat(1).take(128)).non_zero_count_as_item(),
            Err(StatsError::CouldNotConvert {
                from: DataType::Usize,
                to: DataType::Item
            })
        )
    }

    #[test]
    fn test_std_dev_vec_fail() {
        assert_eq!(
            Vec::<i32>::new().std_dev(),
            Err(StatsError::EmptyCollection)
        )
    }
}
