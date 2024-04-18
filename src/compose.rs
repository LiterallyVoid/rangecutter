use std::ops::Range;

pub trait RangeCompose<Rhs> {
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
