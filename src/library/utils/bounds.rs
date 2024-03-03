use std::cmp::Ordering;

pub trait LowerBound<T> {
    fn lower_bound(&self, x: &T) -> usize;
}

impl<T: Ord> LowerBound<T> for [T] {
    fn lower_bound(&self, x: &T) -> usize {
        self.binary_search_by(|y| match y.cmp(x) {
            Ordering::Equal => Ordering::Greater,
            other => other,
        })
        .unwrap_err()
    }
}

pub trait UpperBound<T> {
    fn upper_bound(&self, x: &T) -> usize;
}

impl<T: Ord> UpperBound<T> for [T] {
    fn upper_bound(&self, x: &T) -> usize {
        self.binary_search_by(|y| match y.cmp(x) {
            Ordering::Equal => Ordering::Less,
            other => other,
        })
        .unwrap_err()
    }
}
