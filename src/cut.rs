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
