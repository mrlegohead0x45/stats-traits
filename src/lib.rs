//! Library for calculating statistics on collections of numbers.

use std::{iter::Sum, ops::Div};

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
    fn count(&self) -> usize {
        self.clone().into_iter().count()
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
    fn mean(&self) -> Self::Item
    where
        Self::Item: Sum + From<usize> + Div<Self::Item, Output = Self::Item>,
    {
        self.sum() / self.count().into()
    }

    /// Like [`Stats::mean`], but returns `None` if the collection is empty.
    /// The [`Stats::mean`] method will panic if the collection is empty.
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
        Self::Item: Sum + From<usize> + Div<Self::Item, Output = Self::Item>,
    {
        if self.count() == 0 {
            None
        } else {
            Some(self.mean())
        }
    }
}

/// Blanket implementation for all types that implement IntoIterator and Clone.
/// This allows us to use the methods on any type that implements those traits.
/// For example, we can use the methods on [`Vec`] and `&[i32]`.
impl<T> Stats for T where T: IntoIterator + Clone {}
