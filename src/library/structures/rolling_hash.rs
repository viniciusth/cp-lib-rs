use std::{cell::RefCell, rc::Rc};

use crate::library::numerical::{functions::modpow, xorshiftrplus::XRng};

// Some large prime numbers, got from http://compoasso.free.fr/primelistweb/page/prime/liste_online_en.php
const MODULOS: &[u64] = &[
    1000000007, 1000000009, 1000000021, 1000000033, 1000000087, 1000000093, 1000000097, 1000000103,
    1000000123, 1000000181, 1000000207, 1000000223, 1000000241, 1000000271, 1000000289, 1000000297,
    1000000321, 1000000349, 1000000363, 1000000403, 1000000409, 1000000411, 1000000427, 1000000433,
    1000000439, 1000000447, 1000000453, 1000000459, 1000000483, 1000000513, 1000000531, 1000000579,
    1000000607, 1000000613, 1000000637, 1000000663, 1000000711, 1000000753, 1000000787, 1000000801,
    1000000829, 1000000861, 1000000871, 1000000891, 1000000901, 1000000919, 1000000931, 1000000933,
    1000000993, 1000001011, 1000001021, 1000001053, 1000001087, 1000001099, 1000001137, 1000001161,
];

type RRC<T> = Rc<RefCell<T>>;

pub struct RollingHash<const R: usize = 1> {
    /// hash[i][j] = \sum_{k=0}^{j-1} input[k] * base[i]^k mod modulos[i]
    hash: [Vec<u64>; R],
    // we don't need to clone the ipowers for every hash struct, so we store them in a shared space.
    ipowers: [RRC<Vec<u64>>; R],
    bases: [u64; R],
    modulos: [u64; R],
}

impl<const R: usize> RollingHash<R> {
    pub fn new(input: &[u64]) -> Self {
        assert!(
            R <= MODULOS.len(),
            "R must be less than or equal to the number of MODULOS"
        );
        let mut rng = XRng::new();
        let mut modulos = MODULOS.to_vec();
        rng.shuffle(&mut modulos);
        let mut bases = vec![0; R];
        let mut hash = vec![vec![0; input.len()]; R];
        let mut ipowers = vec![vec![0; input.len()]; R];
        for i in 0..R {
            let base = rng.next() >> 32;
            bases[i] = base;
            let mut p = 1;
            let ipow = modpow(base as i64, modulos[i] as i64 - 2, modulos[i] as i64) as u64;
            ipowers[i][0] = 1;
            hash[i][0] = input[0];
            for j in 1..input.len() {
                p = p * base % modulos[i];
                ipowers[i][j] = ipowers[i][j - 1] * ipow % modulos[i];
                hash[i][j] = (hash[i][j - 1] + input[j] * p) % modulos[i];
            }
        }

        Self {
            hash: hash.try_into().unwrap(),
            ipowers: ipowers
                .into_iter()
                .map(|x| Rc::new(RefCell::new(x)))
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
            bases: bases.try_into().unwrap(),
            modulos: modulos[..R].try_into().unwrap(),
        }
    }

    pub fn hash(&self, input: &[u64]) -> Self {
        let mut hash = vec![vec![0; input.len()]; R];
        for i in 0..R {
            let base = self.bases[i];
            let ip = self.ipowers[i].borrow()[1];
            let mut p = 1;
            hash[i][0] = input[0];
            for j in 1..input.len() {
                if j == self.ipowers[i].borrow().len() {
                    let Some(&last) = self.ipowers[i].borrow().last() else {
                        unreachable!();
                    };
                    self.ipowers[i].borrow_mut().push(last * ip % self.modulos[i]);
                }

                p = p * base % self.modulos[i];
                hash[i][j] = (hash[i][j - 1] + input[j] * p) % self.modulos[i];

            }
        }
        Self {
            hash: hash.try_into().unwrap(),
            ipowers: self.ipowers.clone(),
            bases: self.bases.clone(),
            modulos: self.modulos.clone(),
        }
    }

    /// Returns the hash of the range [l, r], 0-indexed.
    pub fn get_hash(&self, l: usize, r: usize) -> [u64; R] {
        assert!(l <= r && r < self.hash[0].len(), "Invalid range");
        let mut res = [0; R];
        for i in 0..R {
            if l == 0 {
                res[i] = self.hash[i][r];
                continue;
            }
            // preemptively add modulos[i] to avoid negative numbers
            res[i] = self.hash[i][r] + self.modulos[i] - self.hash[i][l - 1];
            if res[i] >= self.modulos[i] {
                res[i] -= self.modulos[i];
            }
            // you now have \sum_{k=l}^{r} input[k] * base[i]^k mod modulos[i], so divide it by
            // base[i]^l to have a correct hash
            res[i] = res[i] * self.ipowers[i].borrow()[l] % self.modulos[i];
        }
        res
    }
}
