use std::ops::{Div, Mul, Rem};

#[inline]
pub fn gcd<T: Default + Rem<Output = T> + Eq + Copy>(a: T, b: T) -> T {
    if b == T::default() {
        a
    } else {
        gcd(b, a % b)
    }
}

#[inline]
pub fn lcm<T: Default + Rem<Output = T> + Div<Output = T> + Mul<Output = T> + Eq + Copy>(a: T, b: T) -> T {
    a / gcd(a, b) * b
}

/// Returns a vector of pairs (p, cnt) where p is a prime factor
/// and cnt is the number of times it appears
pub fn factorize(mut n: i64) -> Vec<(i64, i64)> {
    let mut res = Vec::new();
    let mut i = 2;
    while i * i <= n {
        let mut cnt = 0;
        while n % i == 0 {
            n = n / i;
            cnt += 1;
        }
        if cnt > 0 {
            res.push((i, cnt));
        }
        i += 1;
    }
    if n > 1 {
        res.push((n, 1));
    }
    res
}

#[inline]
pub fn modpow(mut x: i64, mut n: i64, m: i64) -> i64 {
    assert!(n >= 0, "n must be non-negative");
    assert!(m > 0, "m must be positive");
    assert!((m-1).checked_mul(m-1).is_some(), "(m-1)^2 must fit in i64");
    x %= m;
    let mut res = 1;
    while n > 0 {
        if n & 1 == 1 {
            res = res * x % m;
        }
        x = x * x % m;
        n >>= 1;
    }
    res
}
