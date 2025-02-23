use std::ops::{Mul, MulAssign};

#[derive(Copy, Clone)]
pub struct ModInt<const MOD: u32>(u32);

impl<const MOD: u32> ModInt<MOD> {
    pub fn new(x: u64) -> Self {
        Self((x % (MOD as i64)) as u32)
    }

    pub fn pow(self, mut e: u64) -> Self {
        let mut res = Self::new(1);
        let mut cur = self;
        while e > 0 {
            if e & 1 == 1 {
                res *= cur;
            }
            cur *= cur;
            e >>= 1;
        }
        res
    }

    // pub fn inv(self) -> Self {
    //     self.pow(MOD - 2)
    // }
}

impl<const MOD: u32> std::fmt::Display for ModInt<MOD> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0 as i64)
    }
}

impl<const MOD: u32> MulAssign for ModInt<MOD> {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 = (self.0 as u64 * rhs.0 as u64 % MOD as u64) as u32;
    }
}

impl<const MOD: u32> Mul<ModInt<MOD>> for ModInt<MOD> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.0 as i64 * rhs.0 as i64)
    }
}
