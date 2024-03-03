use std::ops::{Index, IndexMut};

pub struct DVec<T, const DIMS: usize> {
    data: Vec<T>,
    dims: [usize; DIMS],
}

impl<T, const DIMS: usize> DVec<T, DIMS>
where
    T: Default + Clone,
{
    pub fn new(dims: [usize; DIMS]) -> Self {
        Self {
            data: vec![Default::default(); dims.iter().product()],
            dims,
        }
    }
}

impl <T, const DIMS: usize> Index<[usize; DIMS]> for DVec<T, DIMS> {
    type Output = T;

    fn index(&self, index: [usize; DIMS]) -> &Self::Output {
        let mut idx = 0;
        let mut mul = 1;
        for i in 0..DIMS {
            idx += index[i] * mul;
            mul *= self.dims[i];
        }
        &self.data[idx]
    }
}

impl <T, const DIMS: usize> IndexMut<[usize; DIMS]> for DVec<T, DIMS> {
    fn index_mut(&mut self, index: [usize; DIMS]) -> &mut Self::Output {
        let mut idx = 0;
        let mut mul = 1;
        for i in 0..DIMS {
            idx += index[i] * mul;
            mul *= self.dims[i];
        }
        &mut self.data[idx]
    }
}
