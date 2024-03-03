use std::ops::{Add, AddAssign, Mul};

pub mod lazy;

pub trait LazyNode<T>: Copy {
    fn new() -> Self;
    fn add_update(&mut self, value: T);
    fn apply_update(&mut self, l: usize, r: usize) -> T;
    fn merge(&self, other: &Self) -> Self;
    fn value(&self) -> T;
}

#[derive(Copy, Clone)]
pub struct SumNode<T> {
    sum: T,
    lazy: T,
}

pub trait LazyNodeType:
    Default + Copy + AddAssign + Add<Output = Self> + Mul<Output = Self>
{
}

impl<T> LazyNodeType for T where
    T: Default + Copy + AddAssign + Add<Output = Self> + Mul<Output = Self>
{
}

impl<T: LazyNodeType + TryFrom<usize>> LazyNode<T> for SumNode<T> {
    fn new() -> Self {
        Self {
            sum: T::default(),
            lazy: T::default(),
        }
    }

    fn add_update(&mut self, value: T) {
        self.lazy += value;
    }

    fn apply_update(&mut self, l: usize, r: usize) -> T {
        self.sum += self.lazy * T::try_from(r - l + 1).ok().unwrap();
        let tmp = self.lazy;
        self.lazy = T::default();
        tmp
    }

    fn merge(&self, other: &Self) -> Self {
        Self {
            sum: self.sum + other.sum,
            lazy: T::default(),
        }
    }

    fn value(&self) -> T {
        self.sum
    }
}

#[derive(Clone, Copy)]
pub struct MaxNode<T> {
    max: T,
    lazy: T,
}

impl<T: LazyNodeType + Ord> LazyNode<T> for MaxNode<T> {
    fn new() -> Self {
        Self {
            max: T::default(),
            lazy: T::default(),
        }
    }

    fn add_update(&mut self, value: T) {
        self.lazy += value;
    }

    fn apply_update(&mut self, _l: usize, _r: usize) -> T {
        self.max += self.lazy;
        let tmp = self.lazy;
        self.lazy = T::default();
        tmp
    }

    fn merge(&self, other: &Self) -> Self {
        Self {
            max: self.max.max(other.max),
            lazy: T::default(),
        }
    }

    fn value(&self) -> T {
        self.max
    }
}
