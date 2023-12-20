use rayon::iter::{
    IndexedParallelIterator, IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelIterator,
};
use rayon::prelude::*;
use std;
use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, Index, Mul, MulAssign, Neg, ShrAssign, Sub},
};

use crypto_bigint::Invert;
use itertools::{EitherOrBoth::*, Itertools};

use crate::{ntt::*, numbers::NttFieldElement};

pub trait PolynomialFieldElement:
    NttFieldElement
    + Display
    + From<u16>
    + From<u32>
    + From<i32>
    + From<u64>
    + From<u128>
    + From<usize>
    + Clone
    + Copy
    + Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + Mul<Output = Self>
    + MulAssign
    + Div<Output = Self>
    + ShrAssign<usize>
    + Neg<Output = Self>
    + PartialOrd
    + PartialEq
    + Invert<Output = Self>
    + Send
    + Sync
{
}

#[derive(Debug, Clone)]
pub struct Polynomial<T: PolynomialFieldElement> {
    pub coef: Vec<T>,
}

impl<T: PolynomialFieldElement> Polynomial<T> {
    pub fn new(coef: Vec<T>) -> Self {
        let n = coef.len();
        let ZERO = T::from(0_u32);

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

    pub fn mul_brute(self, rhs: Polynomial<T>) -> Polynomial<T> {
        let a = self.len();
        let b = rhs.len();
        let ZERO = T::from(0_u32);

        let mut out: Vec<T> = vec![ZERO; a + b];

        for i in 0..a {
            for j in 0..b {
                let e = i + j;
                out[e] += self.coef[i] * rhs.coef[j];
            }
        }

        Polynomial { coef: out }
    }

    #[cfg(feature = "parallel")]
    pub fn mul(self, rhs: Polynomial<T>, c: &Constants<T>) -> Polynomial<T> {
        let v1_deg = self.degree();
        let v2_deg = rhs.degree();
        let n = (self.len() + rhs.len()).next_power_of_two();
        let ZERO = T::from(0);

        let v1: Vec<T> = vec![ZERO; n - self.len()]
            .into_iter()
            .chain(self.coef.into_iter())
            .collect();
        let v2: Vec<T> = vec![ZERO; n - rhs.len()]
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
    pub fn mul(self, rhs: Polynomial<T>, c: &Constants<T>) -> Polynomial<T> {
        let v1_deg = self.degree();
        let v2_deg = rhs.degree();
        let n = (self.len() + rhs.len()).next_power_of_two();
        let ZERO = T::from(0_u32);

        let v1: Vec<T> = vec![ZERO; n - self.len()]
            .into_iter()
            .chain(self.coef.into_iter())
            .collect();
        let v2: Vec<T> = vec![ZERO; n - rhs.len()]
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
            self.coef[n] = self.coef[n - 1] * T::from(N - n);
        }
        let ZERO = T::from(0);
        self.coef[0] = ZERO;
        let start = self.coef.iter().position(|&x| x != ZERO).unwrap();
        self.coef = self.coef[start..].to_vec();

        self
    }

    pub fn len(&self) -> usize {
        self.coef.len()
    }

    pub fn degree(&self) -> usize {
        let ZERO = T::from(0);
        let start = self.coef.iter().position(|&x| x != ZERO).unwrap();
        self.len() - start - 1
    }

    pub fn max(&self) -> T {
        let mut ans = self.coef[0];

        self.coef[1..].iter().for_each(|&x| {
            if ans < x {
                ans = x;
            }
        });

        ans
    }
}

impl<T: PolynomialFieldElement> Add<Polynomial<T>> for Polynomial<T> {
    type Output = Polynomial<T>;

    fn add(self, rhs: Polynomial<T>) -> Self::Output {
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

impl<T: PolynomialFieldElement> Sub<Polynomial<T>> for Polynomial<T> {
    type Output = Polynomial<T>;

    fn sub(self, rhs: Polynomial<T>) -> Self::Output {
        self + (-rhs)
    }
}

impl<T: PolynomialFieldElement> Neg for Polynomial<T> {
    type Output = Polynomial<T>;

    fn neg(self) -> Self::Output {
        Polynomial {
            coef: self.coef.iter().map(|a| -(*a)).collect(),
        }
    }
}

impl<T: PolynomialFieldElement> Display for Polynomial<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.coef.iter().map(|&x| write!(f, "{} ", x)).collect()
    }
}

impl<T: PolynomialFieldElement> Index<usize> for Polynomial<T> {
    type Output = T;

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

            // let mul = a.mul(b, &c);
            // assert_eq!(mul[0], ONE);
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
