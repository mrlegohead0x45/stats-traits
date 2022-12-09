use num_traits::ToPrimitive;

use crate::error::DataType;
use crate::helpers::MinMax;
use crate::helpers::NumExt;
use crate::Result;
use crate::StatsError;

/// Type alias for a tuple containing a frequency and a value.
type Frequency<T> = (usize, T);

/// Similar to [`Stats`](crate::Stats), but for collections of tuples
/// containing a frequency and a value.
pub trait FrequencyStats<T>: IntoIterator<Item = Frequency<T>> + Clone
where
    T: NumExt,
{
    /// Returns the total number of values in the collection.
    /// This is the sum of all frequencies.
    ///
    /// # Examples
    /// ```
    /// use stats_traits::FrequencyStats;
    /// let vec: Vec<(usize, i32)> = vec![(1, 1), (2, 2)];
    /// assert_eq!(vec.count(), 3);
    /// ```
    fn count(&self) -> usize {
        self.clone().into_iter().map(|(freq, _)| freq).sum()
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
    /// to [`Self::Item`](IntoIterator::Item). Return an error
    /// under the same conditions as [`crate::Stats::non_zero_count`],
    /// or if the length could not be converted to [`Self::Item`](IntoIterator::Item)
    fn non_zero_count_into_item(&self) -> Result<T> {
        T::from_usize(self.non_zero_count()?).ok_or(StatsError::CouldNotConvert {
            from: DataType::Usize,
            to: DataType::Item,
        })
    }

    /// Calculate the sum of all the items in the collection
    /// multiplied by their frequency.
    ///
    /// # Examples
    /// ```
    /// use stats_traits::FrequencyStats;
    /// let vec: Vec<(usize, i32)> = vec![(1, 1), (2, 2)];
    /// assert_eq!(vec.sum(), Ok(5));
    /// ```
    /// ```
    /// use stats_traits::FrequencyStats;
    /// let vec: Vec<(usize, f64)> = vec![(1, 1.0), (2, 2.0)];
    /// assert_eq!(vec.sum(), Ok(5.0));
    /// ```
    ///
    /// # Errors
    /// Returns an error if the frequency could not be converted to [`Self::Item`](IntoIterator::Item)
    fn sum(&self) -> Result<T> {
        let mut sum = T::zero();
        for (freq, val) in self.clone().into_iter() {
            sum = sum
                + val
                    * T::from_usize(freq).ok_or(StatsError::CouldNotConvert {
                        from: DataType::Usize,
                        to: DataType::Item,
                    })?;
        }
        Ok(sum)
    }

    /// Calculate the mean of the collection.
    /// This is the sum of all the items in the collection
    /// multiplied by their frequency, divided by the total number of items.
    /// This is the same as [`crate::Stats::mean`], but for collections of tuples
    /// containing a frequency and a value.
    fn mean(&self) -> Result<T> {
        Ok(self.sum()? / self.non_zero_count_into_item()?)
    }

    /// Calculate the variance of the collection.
    /// See [`crate::Stats::variance`] or
    /// [Wikipedia](<https://en.wikipedia.org/wiki/Variance>) for more information.
    fn variance(&self) -> Result<T> {
        let mean = self.mean()?;
        let mut sum = T::zero();

        // tried using .map() and .sum() but it didn't work
        for (freq, val) in self.clone().into_iter() {
            let diff = val - mean;
            // don't use += as that would require T to implement AddAssign
            sum = sum
                + diff
                    * diff
                    * T::from_usize(freq).ok_or(StatsError::CouldNotConvert {
                        from: DataType::Usize,
                        to: DataType::Item,
                    })?;
        }
        Ok(sum / self.non_zero_count_into_item()?)
    }

    /// Calculate the standard deviation of the collection.
    ///
    /// See [`crate::Stats::std_dev`]
    fn std_dev(&self) -> Result<T>
    where
        T: ToPrimitive,
    {
        T::from_f64(
            self.variance()?
                .to_f64()
                .ok_or(StatsError::CouldNotConvert {
                    from: DataType::Item,
                    to: DataType::F64,
                })?
                .sqrt(),
        )
        .ok_or(StatsError::CouldNotConvert {
            from: DataType::F64,
            to: DataType::Item,
        })
    }

    /// Return the smallest value in the collection
    fn min(&self) -> Result<T>
    where
        T: MinMax,
    {
        self.clone()
            .into_iter()
            .map(|(_, val)| val)
            .reduce(T::min)
            .ok_or(StatsError::EmptyCollection)
    }

    /// Returns the largest value in the collection
    fn max(&self) -> Result<T>
    where
        T: MinMax,
    {
        self.clone()
            .into_iter()
            .map(|(_, val)| val)
            .reduce(T::max)
            .ok_or(StatsError::EmptyCollection)
    }

    /// Return the range of the collection
    /// (the largest - the smallest)
    #[inline]
    fn range(&self) -> Result<T>
    where
        T: MinMax,
    {
        Ok(self.max()? - self.min()?)
    }

    /// Return the most frequently occurring value in the collection,
    /// which is the one with the highest frequency
    fn mode(&self) -> Result<T> {
        /*
            tried the following
            self
                .clone()
                .into_iter()
                .max_by_key(|(freq, _)| freq) <- complains about lifetimes
                .map(|(_, val)| val)
                .ok_or(StatsError::EmptyCollection)
        */
        let mut mode = (0, T::zero());
        self.non_zero_count()?;
        for (freq, val) in self.clone() {
            if freq >= mode.0 {
                mode = (freq, val);
            }
        }
        Ok(mode.1)
    }
}

impl<T, I> FrequencyStats<T> for I
where
    I: IntoIterator<Item = Frequency<T>> + Clone,
    T: NumExt,
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
    fn test_non_zero_count() {
        let vec: Vec<(usize, i32)> = vec![(1, 1), (2, 2)];
        assert_eq!(vec.non_zero_count(), Ok(3));
    }

    #[test]
    fn test_sum() {
        let vec: Vec<(usize, i32)> = vec![(1, 1), (2, 2)];
        assert_eq!(vec.sum(), Ok(5));
    }

    #[test]
    fn test_mean() {
        let vec: Vec<(usize, i32)> = vec![(1, 1), (2, 2)];
        assert_eq!(vec.mean(), Ok(1));

        let vec: Vec<(usize, f32)> = vec![(1, 1.0), (2, 2.0)];
        assert_relative_eq!(vec.mean().unwrap(), 5.0 / 3.0);
    }

    #[test]
    fn test_variance() {
        let vec: Vec<(usize, i32)> = vec![(1, 1), (2, 2)];
        assert_eq!(vec.variance(), Ok(0));

        let vec: Vec<(usize, f32)> = vec![(1, 1.0), (2, 2.0)];
        assert_relative_eq!(vec.variance().unwrap(), 2.0 / 9.0);
    }

    #[test]
    fn test_std_dev() {
        let vec: Vec<(usize, i32)> = vec![(1, 1), (2, 2)];
        assert_eq!(vec.std_dev(), Ok(0));
    }
}
