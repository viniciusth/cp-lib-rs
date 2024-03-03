pub trait PairSplit<T> {
    fn pair_split<F>(&self, pred: F) -> SplitByPairs<'_, T, F>
    where
        F: FnMut(&T, &T) -> bool;
}

pub struct SplitByPairs<'a, T: 'a, P>
where
    P: FnMut(&T, &T) -> bool,
{
    v: &'a [T],
    pred: P,
}

impl<'a, T, P> Iterator for SplitByPairs<'a, T, P>
where
    P: FnMut(&T, &T) -> bool,
{
    type Item = &'a [T];

    #[inline]
    fn next(&mut self) -> Option<&'a [T]> {
        if self.v.is_empty() {
            return None;
        }

        let mut idx = 1;
        while idx < self.v.len() && !(self.pred)(&self.v[idx - 1], &self.v[idx]) {
            idx += 1;
        }

        let (left, right) = self.v.split_at(idx);
        self.v = right;
        Some(left)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.v.len()))
    }
}

impl<T> PairSplit<T> for [T] {
    fn pair_split<F>(&self, pred: F) -> SplitByPairs<'_, T, F>
    where
        F: FnMut(&T, &T) -> bool,
    {
        SplitByPairs { v: self, pred }
    }
}
