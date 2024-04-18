use std::ops::{Range, RangeFrom, RangeInclusive, RangeTo, RangeToInclusive};

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

trait RangeCompose<Rhs> {
    type Output;

    fn compose(&self, rhs: &Rhs) -> Self::Output;
}

impl<T> RangeCompose<Range<T>> for Range<T>
where
    T: std::cmp::PartialOrd + std::ops::Add<T, Output = T> + Clone,
{
    type Output = Range<T>;

    fn compose(&self, rhs: &Range<T>) -> Self::Output {
        assert!(self.start.clone() <= self.start.clone() + rhs.start.clone());
        assert!((self.start.clone() + rhs.end.clone()) <= self.end.clone());

        (self.start.clone() + rhs.start.clone())..(self.start.clone() + rhs.end.clone())
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
    ///
    /// ```should_panic
    /// # use rangecutter::RangeExt;
    /// println!("{:?}", (0..1).concat(3..4));
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
        Self: range_cut::RangeCut<C>,
    {
        range_cut::RangeCut::cut(self, middle)
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
        Self: RangeCompose<Rhs, Output = Output>,
    {
        RangeCompose::compose(self, rhs)
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

mod range_cut {
    use super::RangeContainsExt;
    use std::ops::Range;

    pub trait RangeCut<Cut> {
        fn cut(self, middle: &Cut) -> (Self, Self)
        where
            Self: Sized;
    }

    impl<T> RangeCut<Range<T>> for Range<T>
    where
        T: PartialOrd + Clone,
    {
        fn cut(self, middle: &Self) -> (Self, Self) {
            assert!(self.contains(&middle.start));
            assert!(self.contains_or_ends_at(&middle.end));
            assert!(middle.start < middle.end);

            assert!(self.start <= middle.start);

            (
                self.start..middle.start.clone(),
                middle.end.clone()..self.end,
            )
        }
    }
}

// impl<T> RangeCut<T> for Range<T>
// where
//     T: std::cmp::PartialOrd,
// {
//     fn cut(self, middle: T) -> (Self, Self) {
//         assert!(self.contains(&middle));
//         assert!(self.contains_or_ends_at(&middle));

//         (self.start..middle, middle..self.end)
//     }
// }

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
