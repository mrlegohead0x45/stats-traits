use crate::NumExt;

/// Type alias for a tuple containing a frequency and a value.
type Frequency<T> = (usize, T);

/// Similar to [`Stats`](crate::Stats), but for collections of tuples
/// containing a frequency and a value.
pub trait FrequencyStats<T>: IntoIterator<Item = Frequency<T>> + Clone
where
    T: NumExt,
{
    fn count(&self) -> usize {
        self.clone().into_iter().map(|(freq, _)| freq).sum()
    }

    fn sum(&self) -> T {
        self.clone()
            .into_iter()
            .map(|(freq, val)| val * T::from_usize(freq).unwrap())
            .sum()
    }

    fn mean(&self) -> T {
        self.sum() / T::from_usize(self.count()).unwrap()
    }

    fn variance(&self) -> T {
        let mean = self.mean();
        self.clone()
            .into_iter()
            .map(|(freq, val)| {
                let diff = val - mean;
                diff * diff * T::from_usize(freq).unwrap()
            })
            .sum::<T>()
            / T::from_usize(self.count()).unwrap()
    }
}
