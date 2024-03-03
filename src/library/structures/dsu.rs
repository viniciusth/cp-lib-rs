pub struct DSU {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl DSU {
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] == x {
            return x;
        }
        let p = self.parent[x];
        self.parent[x] = self.find(p);
        self.parent[x]
    }

    pub fn merge(&mut self, x: usize, y: usize) -> bool {
        let mut x = self.find(x);
        let mut y = self.find(y);
        if x == y {
            return false;
        }
        if self.size[x] < self.size[y] {
            std::mem::swap(&mut x, &mut y);
        }
        self.parent[y] = x;
        self.size[x] += self.size[y];
        true
    }

    pub fn size(&mut self, x: usize) -> usize {
        let p = self.find(x);
        self.size[p]
    }
}

pub struct BipartiteDSU {
    parent: Vec<usize>,
    size: Vec<usize>,
    len: Vec<i32>,
}

impl BipartiteDSU {
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
            len: vec![0; n],
        }
    }

    pub fn find(&mut self, x: usize) -> (usize, i32) {
        if self.parent[x] == x {
            return (x, 0);
        }
        let (p, len) = self.find(self.parent[x]);
        self.parent[x] = p;
        self.len[x] ^= len;
        (self.parent[x], self.len[x])
    }

    pub fn merge(&mut self, x: usize, y: usize) -> bool {
        let (mut x, px) = self.find(x);
        let (mut y, py) = self.find(y);
        if x == y {
            return px != py;
        }
        if self.size[x] < self.size[y] {
            std::mem::swap(&mut x, &mut y);
        }
        self.parent[y] = x;
        self.size[x] += self.size[y];
        self.len[y] = 1 ^ px ^ py;
        true
    }
}
