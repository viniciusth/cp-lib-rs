struct MergeSortTree {
    n: usize,
    tree: Vec<Vec<(i64, i64)>>,
}

impl MergeSortTree {
    fn new(n: usize, arr: &[i64]) -> Self {
        let mut tree = vec![vec![]; 4 * n];
        Self::build(0, 0, n - 1, arr, &mut tree);
        Self {
            n,
            tree: tree
                .into_iter()
                .map(|x| {
                    let mut y = Vec::with_capacity(x.len());
                    let mut sum = 0;
                    for val in x {
                        sum += val;
                        y.push((val, sum));
                    }
                    y
                })
                .collect(),
        }
    }

    fn build(v: usize, l: usize, r: usize, arr: &[i64], tree: &mut Vec<Vec<i64>>) {
        if l == r {
            tree[v].push(arr[l]);
            return;
        }
        let m = (l + r) / 2;
        Self::build(2 * v + 1, l, m, arr, tree);
        Self::build(2 * v + 2, m + 1, r, arr, tree);
        let mut i = 0;
        let mut j = 0;
        while i < tree[2 * v + 1].len() && j < tree[2 * v + 2].len() {
            let x = tree[2 * v + 1][i];
            let y = tree[2 * v + 2][j];
            if x < y {
                tree[v].push(x);
                i += 1;
            } else {
                tree[v].push(y);
                j += 1;
            }
        }
        while i < tree[2 * v + 1].len() {
            let x = tree[2 * v + 1][i];
            tree[v].push(x);
            i += 1;
        }
        while j < tree[2 * v + 2].len() {
            let y = tree[2 * v + 2][j];
            tree[v].push(y);
            j += 1;
        }
    }

    fn query_impl(&self, v: usize, l: usize, r: usize, ql: usize, qr: usize, x: i64) -> i64 {
        if r < ql || qr < l {
            return 0;
        }
        if ql <= l && r <= qr {
            let mut lo = 0;
            let mut hi = self.tree[v].len();
            while lo < hi {
                let m = (lo + hi) / 2;
                if self.tree[v][m].0 <= x {
                    lo = m + 1;
                } else {
                    hi = m;
                }
            }
            if lo == 0 {
                return 0;
            }
            return self.tree[v][lo - 1].1;
        }
        let m = (l + r) / 2;
        self.query_impl(2 * v + 1, l, m, ql, qr, x)
            + self.query_impl(2 * v + 2, m + 1, r, ql, qr, x)
    }

    fn query(&self, l: usize, r: usize, x: i64) -> i64 {
        self.query_impl(0, 0, self.n - 1, l, r, x)
    }
}
