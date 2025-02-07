
pub struct Sieve {
    n: i32,
    primes: Vec<i32>,
    lo: Vec<i32>,
}

impl Sieve {
    pub fn new(n: i32) -> Self {
        let mut primes = Vec::new();
        let mut lo = vec![0; n as usize + 1];
        for i in 2..=n {
            if lo[i as usize] == 0 {
                lo[i as usize] = i;
                primes.push(i);
            }
            for &p in &primes {
                if p > lo[i as usize] || i * p > n {
                    break;
                }
                lo[(i * p) as usize] = p;
            }
        }
        Self { n, primes, lo }
    }

    pub fn is_prime(&self, x: i32) -> bool {
        assert!(x <= self.n);
        x > 1 && self.lo[x as usize] == x
    }

    pub fn primes(&self) -> &[i32] {
        &self.primes
    }

    pub fn factorize(&self, mut x: i32) -> Vec<(i32, i32)> {
        assert!(x <= self.n);
        let mut res = Vec::new();
        while x > 1 {
            let p = self.lo[x as usize];
            let mut cnt = 0;
            while self.lo[x as usize] == p {
                x /= p;
                cnt += 1;
            }
            res.push((p, cnt));
        }
        res
    }
}
