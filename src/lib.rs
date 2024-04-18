//!
//!

use std::ops::Range;

mod compose;
mod cut;

trait RangeContainsExt<T> {
    fn contains_or_ends_at(&self, index: &T) -> bool;
}

impl<T> RangeContainsExt<T> for Range<T>
where
    T: PartialOrd,
{
    fn contains_or_ends_at(&self, index: &T) -> bool {
        index <= &self.end
    }
}

pub trait RangeExt {
    /// Concatenate `self` and `after`, panicking if `after` doesn't immediately follow `self`.
    ///
    /// ```rust
    /// # use rangecutter::RangeExt;
    /// assert_eq!((0..3).concat(3..4), 0..4);
    ///
    /// let arr = [0, 1, 2, 3, 4];
    ///
    /// assert_eq!([0, 1, 2      ], arr[0..3]);
    /// assert_eq!([         3   ], arr[3..4]);
    /// assert_eq!([0, 1, 2, 3   ], arr[(0..3).concat(3..4)]);
    /// ```
    fn concat(self, after: Self) -> Self;

    /// Remove `prefix` from `self`, panicking if it isn't a prefix of `self`.
    ///
    /// ```rust
    /// # use rangecutter::RangeExt;
    /// assert_eq!((0..5).remove_prefix(0..1), 1..5);
    ///
    /// let arr = [0, 1, 2];
    ///
    /// assert_eq!([0, 1, 2], arr[0..3]);
    /// assert_eq!([0      ], arr[0..1]);
    /// assert_eq!([   1, 2], arr[(0..3).remove_prefix(0..1)]);
    /// ```
    fn remove_prefix(self, prefix: Self) -> Self;

    /// Remove `suffix` from `self`, panicking if it isn't a suffix of `self`.
    ///
    /// ```rust
    /// # use rangecutter::RangeExt;
    /// assert_eq!((0..5).remove_suffix(3..5), 0..3);
    ///
    /// let arr = [0, 1, 2];
    ///
    /// assert_eq!([0, 1, 2], arr[0..3]);
    /// assert_eq!([      2], arr[2..3]);
    /// assert_eq!([0, 1   ], arr[(0..3).remove_suffix(2..3)]);
    /// ```
    fn remove_suffix(self, suffix: Self) -> Self;

    /// Split the range into the section before `middle` starts, and the section that starts where `middle` ends.
    /// Panics if `middle` contains any elements not in `self`.
    ///
    /// ```rust
    /// # use rangecutter::RangeExt;
    /// assert_eq!((0..5).cut(&(1..3)), (0..1, 3..5));
    ///
    /// let arr = [0, 1, 2, 3, 4];
    ///
    /// let middle = 1..3;
    /// let (before, after) = (0..5).cut(&middle);
    ///
    /// assert_eq!(
    ///     arr[before]
    ///         .into_iter()
    ///         .chain(arr[after].into_iter())
    ///         .copied()
    ///         .collect::<Vec<_>>(),
    ///     [0,       3, 4],
    /// );
    ///
    /// assert_eq!(
    ///     arr[middle],
    ///     [   1, 2      ],
    /// );
    /// ```
    fn cut<C>(self, middle: &C) -> (Self, Self)
    where
        Self: Sized,
        Self: cut::RangeCut<C>,
    {
        cut::RangeCut::cut(self, middle)
    }

    /// Calculate the range such that indexing by the new range has the same result as indexing by `self` and then `rhs`.
    ///
    /// ```rust
    /// # use rangecutter::RangeExt;
    /// let arr = [0, 1, 2, 3, 4];
    ///
    /// let outer = 2..4;
    /// let inner = 1..2;
    ///
    /// assert_eq!(arr[outer.compose(&inner)], arr[outer][inner]);
    /// ```
    fn compose<Rhs, Output>(&self, rhs: &Rhs) -> Output
    where
        Self: compose::RangeCompose<Rhs, Output = Output>,
    {
        compose::RangeCompose::compose(self, rhs)
    }
}

impl<T> RangeExt for Range<T>
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
