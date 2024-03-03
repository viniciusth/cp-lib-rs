use std::ops::Rem;

#[inline]
pub fn gcd<T: Default + Rem<Output = T> + Eq + Copy>(a: T, b: T) -> T {
    if b == T::default() {
        a
    } else {
        gcd(b, a % b)
    }
}
