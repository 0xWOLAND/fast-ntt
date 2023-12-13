use std::{
    fmt::Display,
    ops::{Add, Index, Mul, Neg, Sub},
};

use itertools::{EitherOrBoth::*, Itertools};
use rayon::iter::{
    IndexedParallelIterator, IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelIterator,
};

use crate::{ntt::*, numbers::BigInt};

#[derive(Debug, Clone)]
pub struct Polynomial {
    pub coef: Vec<BigInt>,
}

impl Polynomial {
    pub fn new(coef: Vec<BigInt>) -> Self {
        let n = coef.len();
        let ZERO = BigInt::from(0);

        // if is not power of 2
        if !(n & (n - 1) == 0) {
            let pad = n.next_power_of_two() - n;
            return Self {
                coef: vec![ZERO; pad]
                    .into_iter()
                    .chain(coef.into_iter())
                    .collect_vec(),
            };
        }
        Self { coef }
    }

    pub fn mul_brute(self, rhs: Polynomial) -> Polynomial {
        let a = self.len();
        let b = rhs.len();
        let ZERO = BigInt::from(0);

        let mut out: Vec<BigInt> = vec![ZERO; a + b];

        for i in 0..a {
            for j in 0..b {
                let e = i + j;
                out[e] += self.coef[i] * rhs.coef[j];
            }
        }

        Polynomial { coef: out }
    }

    #[cfg(feature = "parallel")]
    pub fn mul(self, rhs: Polynomial, c: &Constants) -> Polynomial {
        let v1_deg = self.degree();
        let v2_deg = rhs.degree();
        let n = (self.len() + rhs.len()).next_power_of_two();
        let ZERO = BigInt::from(0);

        let v1 = vec![ZERO; n - self.len()]
            .into_iter()
            .chain(self.coef.into_iter())
            .collect();
        let v2 = vec![ZERO; n - rhs.len()]
            .into_iter()
            .chain(rhs.coef.into_iter())
            .collect();

        let a_forward = forward(v1, &c);
        let b_forward = forward(v2, &c);

        let mut mul = vec![ZERO; n as usize];
        mul.par_iter_mut()
            .enumerate()
            .for_each(|(i, x)| *x = (a_forward[i] * b_forward[i]).rem(c.N));

        let coef = inverse(mul, &c);
        // n - polynomial degree - 1
        let start = n - (v1_deg + v2_deg + 1) - 1;
        Polynomial {
            coef: coef[start..=(start + v1_deg + v2_deg)].to_vec(),
        }
    }

    #[cfg(not(feature = "parallel"))]
    pub fn mul(self, rhs: Polynomial, c: &Constants) -> Polynomial {
        let v1_deg = self.degree();
        let v2_deg = rhs.degree();
        let n = (self.len() + rhs.len()).next_power_of_two();
        let ZERO = BigInt::from(0);

        let v1 = vec![ZERO; n - self.len()]
            .into_iter()
            .chain(self.coef.into_iter())
            .collect();
        let v2 = vec![ZERO; n - rhs.len()]
            .into_iter()
            .chain(rhs.coef.into_iter())
            .collect();

        let a_forward = forward(v1, &c);
        let b_forward = forward(v2, &c);

        let mut mul = vec![ZERO; n as usize];
        mul.iter_mut()
            .enumerate()
            .for_each(|(i, x)| *x = (a_forward[i] * b_forward[i]).rem(c.N));

        let coef = inverse(mul, &c);
        // n - polynomial degree - 1
        let start = n - (v1_deg + v2_deg + 1) - 1;
        let res = Polynomial {
            coef: coef[start..=(start + v1_deg + v2_deg)].to_vec(),
        };
        res
    }

    pub fn diff(mut self) -> Self {
        let N = self.len();
        for n in (1..N).rev() {
            self.coef[n] = self.coef[n - 1] * BigInt::from(N - n);
        }
        self.coef[0] = BigInt::from(0);
        let start = self.coef.iter().position(|&x| x != 0).unwrap();
        self.coef = self.coef[start..].to_vec();

        self
    }

    pub fn len(&self) -> usize {
        self.coef.len()
    }

    pub fn degree(&self) -> usize {
        let start = self.coef.iter().position(|&x| x != 0).unwrap();
        self.len() - start - 1
    }

    pub fn max(&self) -> BigInt {
        let mut ans = self.coef[0];

        self.coef[1..].iter().for_each(|&x| {
            if ans < x {
                ans = x;
            }
        });

        ans
    }
}

impl Add<Polynomial> for Polynomial {
    type Output = Polynomial;

    fn add(self, rhs: Polynomial) -> Self::Output {
        Polynomial {
            coef: self
                .coef
                .iter()
                .rev()
                .zip_longest(rhs.coef.iter().rev())
                .map(|p| match p {
                    Both(&a, &b) => a + b,
                    Left(&a) => a,
                    Right(&b) => b,
                })
                .rev()
                .collect(),
        }
    }
}

impl Sub<Polynomial> for Polynomial {
    type Output = Polynomial;

    fn sub(self, rhs: Polynomial) -> Self::Output {
        self + (-rhs)
    }
}

impl Neg for Polynomial {
    type Output = Polynomial;

    fn neg(self) -> Self::Output {
        Polynomial {
            coef: self.coef.iter().map(|a| -(*a)).collect(),
        }
    }
}

impl Display for Polynomial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.coef.iter().map(|&x| write!(f, "{} ", x)).collect()
    }
}

impl Index<usize> for Polynomial {
    type Output = BigInt;

    fn index(&self, index: usize) -> &Self::Output {
        &self.coef[index]
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use rand::Rng;

    use super::Polynomial;
    use crate::{
        ntt::{working_modulus, Constants},
        numbers::BigInt,
    };

    #[test]
    fn add() {
        let a = Polynomial::new(vec![1, 2, 3, 4].iter().map(|&x| BigInt::from(x)).collect());
        let b = Polynomial::new(vec![1, 2].iter().map(|&x| BigInt::from(x)).collect());
        println!("{}", a + b);
    }

    #[test]
    fn mul() {
        let ONE = BigInt::from(1);
        (0..10).for_each(|_| {
            let n: usize = 1 << rand::thread_rng().gen::<usize>() % (1 << 3);
            let v1: Vec<BigInt> = (0..n)
                .map(|_| BigInt::from(rand::thread_rng().gen::<u32>() % (1 << 6)))
                .collect();
            let v2: Vec<BigInt> = (0..n)
                .map(|_| BigInt::from(rand::thread_rng().gen::<u32>() % (1 << 6)))
                .collect();
            let a = Polynomial::new(vec![ONE].into_iter().chain(v1.into_iter()).collect_vec());
            let b = Polynomial::new(vec![ONE].into_iter().chain(v2.into_iter()).collect_vec());

            let N = BigInt::from((a.len() + b.len()).next_power_of_two());
            let M = (*a
                .coef
                .iter()
                .max()
                .unwrap()
                .max(b.coef.iter().max().unwrap())
                << 1)
                * N
                + 1;
            let c = working_modulus(N, M);

            let mul = a.mul(b, &c);
            assert_eq!(mul[0], ONE);
        });
    }

    #[test]
    fn diff() {
        let a = Polynomial::new(vec![3, 2, 1].iter().map(|&x| BigInt::from(x)).collect());
        let da = a.diff();
        println!("{}", da);
    }

    #[test]
    fn test_comparator() {
        let a = BigInt::from(550338105);
        let b = BigInt::from(1);
        assert!(a > b);
        let p = Polynomial::new(vec![a, b]);
        let hi = p.max();

        assert_eq!(a, hi);
    }
}
