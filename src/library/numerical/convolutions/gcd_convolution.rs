use std::ops::{AddAssign, Div, Mul, Sub};

/// Given an array of frequencies of elements,
/// computes ai = sum a[k] where gcd(k, i) = i
pub fn gcd_convolution<
    T: AddAssign + Copy + Mul<Output = T> + Sub<Output = T> + Div<Output = T> + From<i32>,
>(
    frequencies: &[T],
) -> Vec<T> {
    let n: usize = frequencies.len();
    let mut dp = vec![T::from(0); n];
    for i in (1..n).rev() {
        let mut s = T::from(0);
        let mut rmv = T::from(0);
        for j in (i..n).step_by(i) {
            s += frequencies[j];
            rmv += dp[j];
        }
        dp[i] = (s - T::from(1)) * s / T::from(2) - rmv;
    }
    dp
}
