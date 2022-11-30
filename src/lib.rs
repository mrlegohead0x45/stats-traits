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
    /// # Example
    /// ```
    /// use stats::Stats;
    /// assert_eq!(vec![1, 2, 3].mean(), 2);
    /// ```
    /// ```
    /// use stats::Stats;
    /// assert_eq!(vec![1.0, 2.0, 3.0].mean(), 2.0);
    /// ```
    fn mean(&self) -> Self::Item
    where
        Self::Item: Sum + From<usize> + Div<Self::Item, Output = Self::Item>,
    {
        self.sum() / self.count().into()
    }
}

impl<T> Stats for Vec<T> where T: Clone {}
