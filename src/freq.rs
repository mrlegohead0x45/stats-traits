use crate::error::DataType;
use crate::NumExt;
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
}

impl<T, I> FrequencyStats<T> for I
where
    I: IntoIterator<Item = Frequency<T>> + Clone,
    T: NumExt,
{
}
