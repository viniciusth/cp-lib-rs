#[allow(non_snake_case)]
pub struct HopcroftKarp {
    n: usize,
    m: usize,
    adj: Vec<Vec<usize>>,
    A: Vec<i32>,
    B: Vec<i32>,
}

impl HopcroftKarp {

    pub fn new(n: usize, m: usize) -> Self {
        Self {
            n,
            m,
            adj: vec![vec![]; n],
            A: vec![0; n],
            B: vec![0; m],
        }
    }

    pub fn add_left(&mut self, u: usize, v: usize) {
        self.adj[u].push(v);
    }

    pub fn modify_m(&mut self, m: usize) {
        self.m = m;
    }

    pub fn compute(&mut self) -> (usize, Vec<i32>) {
        let mut res = 0;
        let mut cur = vec![];
        let mut next = vec![];
        let mut btoa = vec![-1; self.m];

        loop {
            self.A.iter_mut().for_each(|x| *x = 0);
            self.B.iter_mut().for_each(|x| *x = 0);
            cur.clear();
            for i in 0..self.m {
                if btoa[i] != -1 {
                    self.A[btoa[i] as usize] = -1;
                }
            }

            for i in 0..self.n {
                if self.A[i] == 0 {
                    cur.push(i);
                }
            }

            for lay in 1.. {
                let mut islast = false;
                next.clear();
                for i in 0..cur.len() {
                    let a = cur[i];
                    for j in 0..self.adj[a].len() {
                        let b = self.adj[a][j];
                        if b >= self.m {
                            continue;
                        }
                        if btoa[b] == -1 {
                            self.B[b] = lay;
                            islast = true;
                        } else if btoa[b] != a as i32 && self.B[b] == 0 {
                            self.B[b] = lay;
                            next.push(btoa[b] as usize);
                        }
                    }
                }
                if islast {
                    break;
                }
                if next.len() == 0 {
                    return (res, btoa);
                }
                for i in 0..next.len() {
                    self.A[next[i] as usize] = lay;
                }
                std::mem::swap(&mut cur, &mut next);
            }

            for i in 0..self.n {
                res += self.dfs(i, 0, &mut btoa);
            }
        }
    }

    fn dfs(&mut self, a: usize, l: i32, btoa: &mut Vec<i32>) -> usize {
        if self.A[a] != l {
            return 0;
        }
        self.A[a] = -1;
        for i in 0..self.adj[a].len() {
            let b = self.adj[a][i];
            if b >= self.m {
                continue;
            }
            if self.B[b] == l+1 {
                self.B[b] = 0;
                if btoa[b] == -1 || self.dfs(btoa[b] as usize, l, btoa) != 0 {
                    btoa[b] = a as i32;
                    return 1;
                }
            }
        }
        return 0;
    }
}


