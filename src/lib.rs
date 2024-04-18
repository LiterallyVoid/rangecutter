//! Tools for working with adjacent ranges.
//!
//! ```rust
//! use std::ops::Range;
//! use rangecutter::RangeExt;
//!
//! let source = "Line 1\nLine 2\n";
//!
//! fn first_newline(s: &str) -> Option<Range<usize>> {
//!     let sep = '\n';
//!
//!     let start = s.find(sep)?;
//!     let end = start + sep.len_utf8();
//!
//!     Some(start..end)
//! }
//!
//! let mut cursor = 0..source.len();
//!
//! let lines_with_endings = std::iter::from_fn(|| {
//!     let separator = match first_newline(&source[cursor.clone()]) {
//!         Some(newline) => cursor.compose(&newline),
//!         None if !cursor.is_empty() => {
//!             cursor.end..cursor.end
//!         }
//!         None => return None,
//!     };
//!
//!     let line;
//!     (line, cursor) = cursor.cut(&separator);
//!
//!     let line_with_separator = line.concat(&separator);
//!
//!     Some(&source[line_with_separator])
//! }).collect::<Vec<_>>();
//!
//! assert_eq!(
//!     lines_with_endings,
//!     [
//!         "Line 1\n",
//!         "Line 2\n",
//!     ],
//! );
//!
//! ```
//!
//! # Limitations
//!
//! Only fully-bounded inclusive ranges (`Range<T>`) are supported.

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
    /// Concatenate `self` and `other`, panicking if `other` doesn't immediately follow `self`.
    ///
    /// ```rust
    /// # use rangecutter::RangeExt;
    /// assert_eq!((0..3).concat(&(3..4)), 0..4);
    ///
    /// let arr = [0, 1, 2, 3, 4];
    ///
    /// assert_eq!([0, 1, 2      ], arr[0..3]);
    /// assert_eq!([         3   ], arr[3..4]);
    /// assert_eq!([0, 1, 2, 3   ], arr[(0..3).concat(&(3..4))]);
    /// ```
    fn concat(&self, other: &Self) -> Self
    where
        Self: Clone;

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
    fn cut<C>(&self, middle: &C) -> (Self, Self)
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
    T: std::cmp::PartialOrd + std::cmp::PartialEq + Clone,

    // Not required at the moment, but I don't want to have to break compatibility later to print the relevant values while panicking.
    T: std::fmt::Debug,
{
    fn concat(&self, other: &Self) -> Self {
        assert!(self.end == other.start);

        self.start.clone()..other.end.clone()
    }

    fn remove_prefix(self, prefix: Self) -> Self {
        assert!(prefix.start == self.start);
        assert!(self.contains_or_ends_at(&prefix.end));

        prefix.end..self.end
    }

    fn remove_suffix(self, suffix: Self) -> Self {
        assert!(self.contains(&suffix.start));
        assert!(self.end == suffix.end);

        self.start..suffix.start
    }
}
