use std::rc::Rc;

type SparseTableOperation<T> = Rc<dyn Fn(T, T) -> T>;

#[derive(Clone)]
pub struct LazySparseTable<T> {
    log2: Vec<usize>,
    table: Vec<Vec<T>>,
    computed: Vec<Vec<bool>>,
    operation: SparseTableOperation<T>,
}

impl<T: Ord + Default + Copy> LazySparseTable<T> {
    pub fn new(n: usize, operation: SparseTableOperation<T>) -> Self {
        let mut log2 = vec![0; n + 1];
        for i in 2..=n {
            log2[i] = log2[i / 2] + 1;
        }
        Self {
            log2,
            table: vec![vec![T::default(); n]; 18],
            computed: vec![vec![false; n]; 18],
            operation,
        }
    }

    pub fn set(&mut self, i: usize, v: T) {
        self.table[0][i] = v;
        self.computed[0][i] = true;
    }

    pub fn get(&mut self, l: usize, r: usize) -> T {
        let k = self.log2[r - l + 1];
        let f = self.get_impl(k, l);
        let s = self.get_impl(k, r + 1 - (1 << k));
        (self.operation)(f, s)
    }

    fn get_impl(&mut self, i: usize, j: usize) -> T {
        if self.computed[i][j] {
            return self.table[i][j];
        }
        let res = self
            .get_impl(i - 1, j)
            .max(self.get_impl(i - 1, j + (1 << (i - 1))));
        self.table[i][j] = res;
        self.computed[i][j] = true;
        res
    }
}
