pub struct XorBasis<const MAX_BITS: usize> {
    pub basis: Vec<u64>,
    pub sz: usize,
}

impl<const MAX_BITS: usize> XorBasis<MAX_BITS> {
    pub fn new() -> Self {
        Self {
            basis: vec![0; MAX_BITS],
            sz: 0,
        }
    }

    pub fn insert(&mut self, mut x: u64) {
        for i in (0..MAX_BITS).rev() {
            if (self.basis[i] >> i & 1) == 0 {
                continue;
            }
            if self.basis[i] == 0 {
                self.basis[i] = x;
                self.sz += 1;
                return;
            }
            x ^= self.basis[i];
        }
    }
}
