use crate::{numbers::BigInt, polynomial::PolynomialFieldElement, prime::is_prime};
use crypto_bigint::Invert;
use itertools::Itertools;
use rayon::prelude::*;

#[derive(Debug, Clone)]
pub struct Constants<T: PolynomialFieldElement> {
    pub N: T,
    pub w: T,
}

fn prime_factors<T: PolynomialFieldElement>(a: T) -> Vec<T> {
    let mut ans: Vec<T> = Vec::new();
    let ZERO = T::from(0);
    let ONE = T::from(1);
    let mut x = T::from(2);
    while x * x <= a {
        if a.rem(x) == ZERO {
            ans.push(x);
        }
        x += ONE;
    }
    ans
}

#[cfg(feature = "parallel")]
fn is_primitive_root<T: PolynomialFieldElement>(a: T, deg: T, N: T) -> bool {
    let lhs = a.mod_exp(deg, N);
    let ONE = T::from(1);
    let lhs = lhs == ONE;
    let rhs = prime_factors(deg)
        .par_iter()
        .map(|&x| a.mod_exp(deg / x, N) != ONE)
        .all(|x| x);
    lhs && rhs
}

#[cfg(not(feature = "parallel"))]
fn is_primitive_root<T: PolynomialFieldElement>(a: T, deg: T, N: T) -> bool {
    let lhs = a.mod_exp(deg, N);
    let ONE = T::from(1);
    let lhs = lhs == ONE;
    let rhs = prime_factors(deg)
        .iter()
        .map(|&x| a.mod_exp(deg / x, N) != ONE)
        .all(|x| x);
    lhs && rhs
}

pub fn working_modulus<T: PolynomialFieldElement>(n: T, M: T) -> Constants<T> {
    let ZERO = T::from(0);
    let ONE = T::from(1);
    let mut N = M;
    if N >= ONE {
        N = N * n + ONE;
        while !is_prime(N) {
            N += n;
        }
    }
    let totient = N - ONE;
    assert!(N >= M);
    let mut gen = T::from(0);
    let mut g = T::from(2);
    while g < N {
        if is_primitive_root(g, totient, N) {
            gen = g;
            break;
        }
        g += ONE;
    }
    assert!(gen > ZERO);
    let w = gen.mod_exp(totient / n, N);
    Constants { N, w }
}

fn order_reverse<T: PolynomialFieldElement>(inp: &mut Vec<T>) {
    let mut j = 0;
    let n = inp.len();
    (1..n).for_each(|i| {
        let mut bit = n >> 1;
        while (j & bit) > 0 {
            j ^= bit;
            bit >>= 1;
        }
        j ^= bit;

        if i < j {
            inp.swap(i, j);
        }
    });
}

#[cfg(feature = "parallel")]
fn fft<T: PolynomialFieldElement>(inp: Vec<T>, c: &Constants<T>, w: T) -> Vec<T> {
    assert!(inp.len().is_power_of_two());
    let mut inp = inp.clone();
    let N = inp.len();
    let MOD = T::from(c.N);
    let ONE = T::from(1);
    let mut pre: Vec<T> = vec![ONE; N / 2];
    let CHUNK_COUNT = 128;
    let chunk_count = T::from(CHUNK_COUNT);

    pre.par_chunks_mut(CHUNK_COUNT)
        .enumerate()
        .for_each(|(i, arr)| arr[0] = w.mod_exp(T::from(i) * chunk_count, MOD));
    pre.par_chunks_mut(CHUNK_COUNT).for_each(|x| {
        (1..x.len()).for_each(|y| {
            let _x = x.to_vec();
            x[y] = (w * x[y - 1]).rem(MOD);
        })
    });
    order_reverse(&mut inp);

    let mut gap = 1;

    while gap < inp.len() {
        let nchunks = inp.len() / (2 * gap);
        inp.par_chunks_mut(2 * gap).for_each(|cxi| {
            let (lo, hi) = cxi.split_at_mut(gap);
            lo.par_iter_mut()
                .zip(hi)
                .enumerate()
                .for_each(|(idx, (lo, hi))| {
                    *hi = (*hi * pre[nchunks * idx]).rem(MOD);
                    let neg = if *lo < *hi {
                        (MOD + *lo) - *hi
                    } else {
                        *lo - *hi
                    };
                    *lo = if *lo + *hi >= MOD {
                        (*lo + *hi) - MOD
                    } else {
                        *lo + *hi
                    };
                    *hi = neg;
                });
        });
        gap *= 2;
    }
    inp
}

#[cfg(not(feature = "parallel"))]
fn fft<T: PolynomialFieldElement>(inp: Vec<T>, c: &Constants<T>, w: T) -> Vec<T> {
    assert!(inp.len().is_power_of_two());
    let mut inp = inp.clone();
    let N = inp.len();
    let MOD = T::from(c.N);
    let ONE = T::from(1);
    let mut pre: Vec<T> = vec![ONE; N / 2];
    let CHUNK_COUNT = 128;
    let chunk_count = T::from(CHUNK_COUNT);

    pre.chunks_mut(CHUNK_COUNT)
        .enumerate()
        .for_each(|(i, arr)| arr[0] = w.mod_exp(T::from(i) * chunk_count, MOD));
    pre.chunks_mut(CHUNK_COUNT).for_each(|x| {
        (1..x.len()).for_each(|y| {
            let _x = x.to_vec();
            x[y] = (w * x[y - 1]).rem(MOD);
        })
    });
    order_reverse(&mut inp);

    let mut gap = 1;

    while gap < inp.len() {
        let nchunks = inp.len() / (2 * gap);
        inp.chunks_mut(2 * gap).for_each(|cxi| {
            let (lo, hi) = cxi.split_at_mut(gap);
            lo.iter_mut()
                .zip(hi)
                .enumerate()
                .for_each(|(idx, (lo, hi))| {
                    *hi = (*hi * pre[nchunks * idx]).rem(MOD);
                    let neg = if *lo < *hi {
                        (MOD + *lo) - *hi
                    } else {
                        *lo - *hi
                    };
                    *lo = if *lo + *hi >= MOD {
                        (*lo + *hi) - MOD
                    } else {
                        *lo + *hi
                    };
                    *hi = neg;
                });
        });
        gap *= 2;
    }
    inp
}

pub fn forward<T: PolynomialFieldElement>(inp: Vec<T>, c: &Constants<T>) -> Vec<T> {
    fft(inp, c, c.w)
}

#[cfg(feature = "parallel")]
pub fn inverse<T: PolynomialFieldElement>(inp: Vec<T>, c: &Constants<T>) -> Vec<T> {
    let mut inv = T::from(inp.len());
    let _ = inv.set_mod(c.N);
    let inv = inv.invert();
    let w = c.w.invert();
    let mut res = fft(inp, c, w);
    res.par_iter_mut().for_each(|x| *x = (inv * (*x)).rem(c.N));
    res
}

#[cfg(not(feature = "parallel"))]
pub fn inverse<T: PolynomialFieldElement>(inp: Vec<T>, c: &Constants<T>) -> Vec<T> {
    let mut inv = T::from(inp.len());
    let _ = inv.set_mod(c.N);
    let inv = inv.invert();
    let w = c.w.invert();
    let mut res = fft(inp, c, w);
    res.iter_mut().for_each(|x| *x = (inv * (*x)).rem(c.N));
    res
}

#[cfg(test)]
mod tests {
    use rand::Rng;
    use rayon::{iter::ParallelIterator, slice::ParallelSliceMut};

    use crate::{
        ntt::{forward, inverse, working_modulus},
        numbers::BigInt,
    };

    #[test]
    fn test_forward() {
        // let n = 1 << rand::thread_rng().gen::<u32>() % 8;
        // let v: Vec<BigInt> = (0..n)
        //     .map(|_| BigInt::from(rand::thread_rng().gen::<u32>() % (1 << 6)))
        //     .collect();
        // let M = (*v.iter().max().unwrap() << 1) * BigInt::from(n) + 1;
        let n = 8;
        let v: Vec<BigInt> = (0..n).map(|x| BigInt::from(x)).collect();
        let M = BigInt::from(n) * BigInt::from(n) + 1;
        let c = working_modulus(BigInt::from(n), BigInt::from(M));
        let forward = forward(v.clone(), &c);
        let inverse = inverse(forward, &c);
        v.iter().zip(inverse).for_each(|(&a, b)| assert_eq!(a, b));
    }

    #[test]
    fn test_roots_of_unity() {
        let N = 10;
        let ONE = BigInt::from(1);
        let mut pre: Vec<BigInt> = vec![ONE; N / 2];
        let mut pre2 = pre.clone();
        let CHUNK_COUNT = 128;
        let MOD = BigInt::from(10);
        let chunk_count = BigInt::from(CHUNK_COUNT);
        let w = BigInt::from(2);

        (1..N / 2).for_each(|i| pre[i] = (pre[i - 1] * w).rem(MOD));

        (1..N / (2 * CHUNK_COUNT))
            .for_each(|i| pre2[i * CHUNK_COUNT] = w.mod_exp(BigInt::from(i) * chunk_count, MOD));
        pre2.par_chunks_mut(CHUNK_COUNT).for_each(|x| {
            (1..x.len()).for_each(|y| {
                let _x = x.to_vec();
                x[y] = (w * x[y - 1]).rem(MOD);
            })
        });
        (0..N / 2).for_each(|i| {
            assert_eq!(pre[i], pre2[i]);
        })
    }
}
