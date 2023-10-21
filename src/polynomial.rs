use std::{
    mem::swap,
    ops::{Add, Mul, Neg, Sub},
};

use itertools::{EitherOrBoth::*, Itertools};

use crate::ntt::*;

#[derive(Debug, Clone)]
pub struct Polynomial {
    pub coef: Vec<i64>,
}

impl Polynomial {
    pub fn new(coef: Vec<i64>) -> Self {
        let n = coef.len();

        // if is not power of 2
        if !(n & (n - 1) == 0) {
            let pad = n.next_power_of_two() - n;
            return Self {
                coef: vec![0; pad]
                    .into_iter()
                    .chain(coef.into_iter())
                    .collect_vec(),
            };
        }
        Self { coef }
    }
    pub fn diff(mut self) -> Self {
        let N = self.coef.len();
        for n in (1..N).rev() {
            self.coef[n] = self.coef[n - 1] * ((N - n) as i64);
        }
        self.coef[0] = 0;
        self.coef = self.coef[1..].to_vec();

        self
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
            coef: self.coef.iter().map(|a| -a).collect(),
        }
    }
}

impl Mul<Polynomial> for Polynomial {
    type Output = Polynomial;

    fn mul(self, rhs: Polynomial) -> Self::Output {
        let mut v1 = self.coef;
        let mut v2 = rhs.coef;
        let n = (v1.len() + v2.len()).next_power_of_two() as i64;
        let v1_deg = v1.len() - 1;
        let v2_deg = v2.len() - 1;

        v1 = vec![0; (n - v1.len() as i64) as usize]
            .into_iter()
            .chain(v1.into_iter())
            .collect();
        v2 = vec![0; (n - v2.len() as i64) as usize]
            .into_iter()
            .chain(v2.into_iter())
            .collect();

        let M = v1
            .iter()
            .map(|x| x.abs())
            .max()
            .unwrap()
            .max(v2.iter().map(|x| x.abs()).max().unwrap())
            .pow(2) as i64
            * n
            + 1;
        let c = working_modulus(n, M);

        v1.iter_mut().for_each(|x| {
            if *x < 0 {
                *x = (*x).rem_euclid(M)
            }
        });
        v2.iter_mut().for_each(|x| {
            if *x < 0 {
                *x = (*x).rem_euclid(M)
            }
        });

        let a_forward = forward(v1, &c);
        let b_forward = forward(v2, &c);

        let mut mul: Vec<i64> = vec![0; n as usize];
        a_forward
            .iter()
            .rev()
            .zip_longest(b_forward.iter().rev())
            .enumerate()
            .for_each(|(i, p)| match p {
                Both(&a, &b) => mul[i] = (a * b) % c.N,
                Left(_) => {}
                Right(_) => {}
            });
        mul.reverse();
        let coef = inverse(mul, &c)
            .iter()
            .map(|&x| if x > M / 2 { -(M - x.rem_euclid(M)) } else { x })
            .collect::<Vec<i64>>()
            .to_vec();
        let start = coef.iter().position(|&x| x != 0).unwrap();

        Polynomial {
            coef: coef[start..(start + v1_deg + v2_deg)].to_vec(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Polynomial;

    #[test]
    fn add() {
        let a = Polynomial::new(vec![1, 2, 3, 4]);
        let b = Polynomial::new(vec![1, 2]);
        println!("{:?}", a + b);
    }

    #[test]
    fn mul() {
        let a = Polynomial::new(vec![1, -2, 3]);
        let b = Polynomial::new(vec![1, -3]);
        println!("{:?}", a * b);
    }

    #[test]
    fn diff() {
        let a = Polynomial::new(vec![3, 2, 1]);
        let da = a.diff();
        println!("{:?}", da);
    }
}
