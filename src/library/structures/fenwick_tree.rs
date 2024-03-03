pub struct PrefixMaxFenwick {
    pub n: usize,
    pub data: Vec<i64>,
}

impl PrefixMaxFenwick {
    pub fn new(n: usize) -> Self {
        Self {
            n,
            data: vec![0; n + 1],
        }
    }

    pub fn add(&mut self, mut i: usize, x: i64) {
        while i <= self.n {
            self.data[i] = self.data[i].max(x);
            i += i & i.wrapping_neg();
        }
    }

    pub fn max(&self, mut i: usize) -> i64 {
        let mut res = 0;
        while i > 0 {
            res = res.max(self.data[i]);
            i -= i & i.wrapping_neg();
        }
        res
    }
}
