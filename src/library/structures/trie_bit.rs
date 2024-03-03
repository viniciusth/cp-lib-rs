pub struct TrieBit<const N: usize> {
    trie: Vec<[u32; 2]>,
    cnt: Vec<u32>,
}

impl<const N: usize> TrieBit<N> {
    pub fn new() -> Self {
        Self {
            trie: vec![[0; 2]; 1],
            cnt: vec![0; 1],
        }
    }

    fn can(&self, idx: usize, bit: i64) -> bool {
        self.trie[idx][bit as usize] != 0 && self.cnt[self.trie[idx][bit as usize] as usize] > 0
    }

    pub fn insert(&mut self, x: i64) {
        let mut v = 0;
        self.cnt[v] += 1;
        for i in (0..N).rev() {
            let bit = (x >> i) & 1;
            if !self.can(v, bit) {
                self.trie.push([0; 2]);
                self.cnt.push(0);
                self.trie[v][bit as usize] = self.trie.len() as u32 - 1;
            }
            v = self.trie[v][bit as usize] as usize;
            self.cnt[v] += 1;
        }
    }

    pub fn remove(&mut self, x: i64) {
        let mut v = 0;
        self.cnt[v] -= 1;
        for i in (0..N).rev() {
            let bit = (x >> i) & 1;
            v = self.trie[v][bit as usize] as usize;
            self.cnt[v] -= 1;
        }
    }

    pub fn max(&self, x: i64) -> i64 {
        let mut v = 0;
        let mut res = 0;
        for i in (0..N).rev() {
            let bit = (x >> i) & 1;
            let other_bit = 1 - bit;
            if self.can(v, other_bit) {
                v = self.trie[v][other_bit as usize] as usize;
                res |= 1 << i;
            } else if self.can(v, bit) {
                v = self.trie[v][bit as usize] as usize;
            } else {
                break;
            }
        }
        res
    }

    pub fn min(&self, x: i64) -> i64 {
        let mut v = 0;
        let mut res = 0;
        for i in (0..N).rev() {
            let bit = (x >> i) & 1;
            if self.can(v, bit) {
                v = self.trie[v][bit as usize] as usize;
            } else if self.can(v, 1 - bit) {
                v = self.trie[v][1 - bit as usize] as usize;
                res |= 1 << i;
            } else {
                break;
            }
        }
        res
    }
}
