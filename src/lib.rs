//! Library for calculating statistics on collections of numbers.

#![warn(missing_docs)]
#![no_std]

use core::{iter::Sum, ops::Div};

use num_traits::{FromPrimitive, Num};

/// A trait to be implemented for collection-like types
/// that provides statistical methods. Requires that the
/// type it is implemented on can be converted into an
/// [`Iterator`].
pub trait Stats: IntoIterator + Clone {
    /// Calculate the sum of all the items in the collection
    ///
    /// # Example
    /// ```
    /// use stats::Stats;
    /// assert_eq!(vec![1, 2, 3].sum(), 6);
    /// ```
    /// ```
    /// use stats::Stats;
    /// assert_eq!(vec![1.0, 2.0, 3.0].sum(), 6.0);
    /// ```
    #[inline]
    fn sum(&self) -> Self::Item
    where
        Self::Item: Sum,
    {
        self.clone().into_iter().sum()
    }

    /// Count the items in the collection
    ///
    /// # Example
    /// ```
    /// use stats::Stats;
    /// assert_eq!(vec![1, 2, 3].count(), 3);
    /// ```
    #[inline]
    fn count(&self) -> usize {
        self.clone().into_iter().count()
    }

    /// Return the length of the collection as a [`Self::Item`].
    /// Will panic if the length is greater than the maximum value
    /// of the type.
    ///
    /// [`Self::Item`]: IntoIterator::Item
    #[inline]
    fn panicking_count(&self) -> Self::Item
    where
        Self::Item: FromPrimitive,
    {
        match Self::Item::from_usize(self.count()) {
            Some(count) => count,
            None => panic!("Could not convert usize to Self::Item"),
        }
    }

    /// Find the mean of the collection
    ///
    /// # Examples
    /// ```
    /// use stats::Stats;
    /// assert_eq!(vec![1, 2, 3].mean(), 2);
    /// ```
    /// ```
    /// use stats::Stats;
    /// let v: Vec<f64> = vec![1.0, 2.0, 3.0];
    /// assert_eq!(v.mean(), 2.0);
    /// ```
    /// Watch out for integer division!
    /// ```
    /// use stats::Stats;
    /// assert_eq!(vec![1, 2, 3, 4].mean(), 2);
    /// ```
    ///
    /// # Panics
    /// Panics if the collection is empty (has a length of 0).
    /// ```
    /// use stats::Stats;
    /// let v: Vec<i64> = vec![];
    /// # let res = std::panic::catch_unwind(|| {
    /// v.mean(); // panics
    /// # });
    /// # assert!(res.is_err());
    /// ```
    /// Will also panic if the length of the collection is too large
    /// to fit in [`Self::Item`](IntoIterator::Item).
    fn mean(&self) -> Self::Item
    where
        Self::Item: Sum + FromPrimitive + Div<Self::Item, Output = Self::Item>,
    {
        self.sum() / self.panicking_count()
    }

    /// Like [`Stats::mean`], but returns `None` if the collection is empty,
    /// or if the length of the collection is too large to fit in [`Self::Item`](IntoIterator::Item),
    /// whereas [`Stats::mean`] method will panic in these cases.
    /// The same caveat about integer division applies.
    ///
    /// # Examples
    /// ```
    /// use stats::Stats;
    /// assert_eq!(vec![1, 2, 3].checked_mean(), Some(2));
    /// ```
    /// ```
    /// use stats::Stats;
    /// assert_eq!(Vec::<i32>::new().checked_mean(), None);
    /// ```
    /// ```
    /// use stats::Stats;
    /// let v: Vec<f64> = vec![1.0, 2.0, 3.0];
    /// assert_eq!(v.checked_mean(), Some(2.0));
    /// ```
    fn checked_mean(&self) -> Option<Self::Item>
    where
        Self::Item: Sum + FromPrimitive + Div<Self::Item, Output = Self::Item>,
    {
        if self.count() == 0 || Self::Item::from_usize(self.count()).is_none() {
            None
        } else {
            Some(self.mean())
        }
    }

    /// Find the variance of the collection.
    /// The variance is the sum of the squared differences between each item
    /// and the mean, divided by the number of items in the collection.
    /// The variance is a measure of how spread out the items are.
    ///
    /// # Examples
    /// ```
    /// use stats::Stats;
    /// assert_eq!(vec![1.0, 2.0, 3.0].variance(), 2.0 / 3.0);
    /// ```
    ///
    /// # Panics
    /// Panics if the collection is empty (has a length of 0).
    /// Will also panic if the length of the collection is too large
    /// to fit in [`Self::Item`](IntoIterator::Item).
    ///
    /// [Wikipedia](<https://en.wikipedia.org/wiki/Variance>)
    fn variance(&self) -> Self::Item
    where
        Self::Item: Num + Sum + FromPrimitive + Copy,
    {
        let mean = self.mean();
        self.clone()
            .into_iter()
            .map(|x| (x - mean) * (x - mean))
            .sum::<Self::Item>()
            / self.panicking_count()
    }
}

/// Blanket implementation for all types that implement [`IntoIterator`] and [`Copy`].
/// This allows us to use the methods on any type that implements those traits.
/// For example, we can use the methods on `Vec` and `&[i32]`.
impl<T> Stats for T where T: IntoIterator + Clone {}

#[cfg(test)]
mod tests {
    use super::*;

    extern crate std;
    use std::prelude::rust_2021::*;
    use std::vec;

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
        assert_eq!(v.mean(), 2);

        let v: Vec<f64> = vec![1.0, 2.0, 3.0];
        assert_eq!(v.mean(), 2.0);

        let v = vec![1, 2, 3, 4];
        assert_eq!(v.mean(), 2);
    }

    #[test]
    fn test_checked_mean_vec() {
        let v = vec![1, 2, 3];
        assert_eq!(v.checked_mean(), Some(2));

        let v = Vec::<i32>::new();
        assert_eq!(v.checked_mean(), None);

        let v = vec![1.0, 2.0, 3.0];
        assert_eq!(v.checked_mean(), Some(2.0));
    }

    #[test]
    #[should_panic]
    fn test_mean_panic() {
        let v = Vec::<i64>::new();
        v.mean();
    }

    #[test]
    #[should_panic]
    fn test_panicking_count() {
        // i8 can hold [-127, 127]
        let v: Vec<i8> = Vec::from_iter(std::iter::repeat(0).take(128));
        v.panicking_count();
    }
}
