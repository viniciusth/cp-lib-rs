pub struct LcaTree {
    pub n: usize,
    pub g: Vec<Vec<(usize, i64)>>,
    pub p: Vec<Vec<usize>>,
    pub cost: Vec<Vec<i64>>,
    pub h: Vec<usize>,
    pub timer: usize,
    pub tin: Vec<usize>,
    pub tout: Vec<usize>,
}

impl LcaTree {
    pub fn new(n: usize) -> Self {
        Self {
            n,
            g: vec![vec![]; n],
            p: vec![vec![n; 20]; n],
            cost: vec![vec![0; 20]; n],
            h: vec![0; n],
            timer: 0,
            tin: vec![0; n],
            tout: vec![0; n],
        }
    }

    pub fn add_edge(&mut self, u: usize, v: usize, w: i64) {
        self.g[u].push((v, w));
        self.g[v].push((u, w));
    }

    pub fn dfs(&mut self, u: usize, par: usize) {
        self.p[u][0] = par;
        self.tin[u] = self.timer;
        self.timer += 1;
        for i in 1..20 {
            self.p[u][i] = self.p[self.p[u][i - 1]][i - 1];
            self.cost[u][i] = self.cost[u][i - 1] + self.cost[self.p[u][i - 1]][i - 1];
        }
        for (v, w) in self.g[u].clone() {
            if v != par {
                self.h[v] = self.h[u] + 1;
                self.cost[v][0] = w;
                self.dfs(v, u);
            }
        }
        self.tout[u] = self.timer;
        self.timer += 1;
    }

    pub fn is_ancestor(&self, u: usize, v: usize) -> bool {
        self.tin[u] <= self.tin[v] && self.tout[u] >= self.tout[v]
    }

    pub fn lca(&self, mut u: usize, v: usize) -> usize {
        if self.is_ancestor(u, v) {
            return u;
        }
        if self.is_ancestor(v, u) {
            return v;
        }
        for i in (0..20).rev() {
            if !self.is_ancestor(self.p[u][i], v) {
                u = self.p[u][i];
            }
        }
        self.p[u][0]
    }

    pub fn get_cost(&self, mut u: usize, mut v: usize) -> i64 {
        let mut res = 0;
        if self.h[u] < self.h[v] {
            std::mem::swap(&mut u, &mut v);
        }
        for i in (0..20).rev() {
            if self.h[self.p[u][i]] >= self.h[v] {
                res += self.cost[u][i];
                u = self.p[u][i];
            }
        }
        if u == v {
            return res;
        }
        for i in (0..20).rev() {
            if self.p[u][i] != self.p[v][i] {
                res += self.cost[u][i] + self.cost[v][i];
                u = self.p[u][i];
                v = self.p[v][i];
            }
        }
        res + self.cost[u][0] + self.cost[v][0]
    }
}
