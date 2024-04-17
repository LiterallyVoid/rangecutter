use std::ops::Range;

pub trait RangeCompose<Rhs> {
    /// Calculate the range such that indexing by the new range has the same result as indexing by `self` and then `rhs`.
    fn compose(&self, rhs: &Rhs) -> Rhs;
}

impl<T> RangeCompose<Range<T>> for Range<T>
where
    T: std::cmp::PartialOrd + std::ops::Add<T, Output = T> + Clone,
{
    fn compose(&self, rhs: &Range<T>) -> Range<T> {
        assert!(self.start.clone() <= self.start.clone() + rhs.start.clone());
        assert!((self.start.clone() + rhs.end.clone()) <= self.end.clone());

        (self.start.clone() + rhs.start.clone())..(self.start.clone() + rhs.end.clone())
    }
}

pub trait RangeCut<Cut> {
    /// Split the range into the section before `middle` starts and the section that starts where `middle` ends.
    /// Panics if `middle` contains any elements not in `self`.
    ///
    /// ```rust
    /// assert_eq((0..5).cut(2), (0..2, 2..5));
    /// ```
    ///
    /// ```rust
    /// assert_eq((0..5).cut(1..3), (0..1, 3..5));
    ///
    /// let arr = [0, 1, 2, 3, 4];
    ///
    /// let middle = 1..3;
    /// let (before, after) = arr.cut(middle);
    ///
    /// assert_eq!(
    ///     arr[before]
    ///         .into_iter()
    ///         .chain(arr[after].into_iter())
    ///         .collect::<Vec<_>>(),
    ///     [0,       3, 4],
    /// );
    ///
    /// assert_eq!(
    ///     arr[middle],
    ///     [   1, 2      ],
    /// );
    /// ```
    fn cut(self, middle: Cut) -> (Self, Self)
    where
        Self: Sized;
}

pub trait RangeAdjacent {
    /// Concatenate `self` and `after`, panicking if `after` doesn't immediately follow `self`.
    ///
    /// ```rust
    /// assert_eq!((0..3).concat(3..4), 0..4);
    ///
    /// let arr = [0, 1, 2, 3, 4];
    ///
    /// assert_eq!([0, 1, 2      ], arr[0..3]);
    /// assert_eq!([         3   ], arr[3..4]);
    /// assert_eq!([0, 1, 2, 3   ], arr[(0..3).concat(3..4)]);
    /// ```
    fn concat(self, after: Self) -> Self;

    /// Remove `prefix` from `self`, panicking if `prefix` isn't a prefix of `self`.
    ///
    /// ```rust
    /// assert_eq!((0..5).remove_prefix(0..1), 1..5);
    ///
    /// let arr = [0, 1, 2];
    ///
    /// assert_eq!([0, 1, 2], arr[0..3]);
    /// assert_eq!([0      ], arr[0..1]);
    /// assert_eq!([   1, 2], arr[(0..3).remove_prefix(0..1)]);
    /// ```
    fn remove_prefix(self, prefix: Self) -> Self;

    /// Remove `suffix` from `self`, panicking if `suffix` isn't a suffix of `self`.
    ///
    /// ```rust
    /// assert_eq!((0..5).remove_suffix(3..5), 0..3);
    ///
    /// let arr = [0, 1, 2];
    ///
    /// assert_eq!([0, 1, 2], arr[0..3]);
    /// assert_eq!([      2], arr[2..3]);
    /// assert_eq!([0, 1   ], arr[(0..3).remove_suffix(2..3)]);
    /// ```
    fn remove_suffix(self, suffix: Self) -> Self;
}

impl<T> RangeAdjacent for Range<T>
where
    T: std::cmp::PartialOrd + std::cmp::PartialEq,
{
    fn concat(self, after: Self) -> Self {
        assert!(self.end == after.start);

        self.start..after.end
    }

    fn remove_prefix(self, prefix: Self) -> Self {
        assert!(prefix.start == self.start);
        assert!(prefix.end <= self.end);

        prefix.end..self.end
    }

    fn remove_suffix(self, suffix: Self) -> Self {
        assert!(self.start <= suffix.start);
        assert!(self.end == suffix.end);

        self.start..suffix.start
    }
}

impl<T> RangeCut<Range<T>> for Range<T>
where
    T: std::cmp::PartialOrd + std::cmp::PartialEq,
{
    fn cut(self, middle: Self) -> (Self, Self) {
        assert!(self.start <= middle.start);
        assert!(middle.end <= self.end);

        (self.start..middle.start, middle.end..self.end)
    }
}

pub trait RangeExt<T>: RangeAdjacent + RangeCompose<T> + RangeCut<T> {}
impl<T> RangeExt<T> for T where T: RangeAdjacent + RangeCompose<T> + RangeCut<T> {}

/// TODO: remove
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
