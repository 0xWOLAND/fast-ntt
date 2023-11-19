use crate::{numbers::BigInt, prime::is_prime};
use itertools::Itertools;
use rayon::prelude::*;

#[derive(Debug, Clone)]
pub struct Constants {
    pub N: BigInt,
    pub w: BigInt,
}

fn extended_gcd(a: BigInt, b: BigInt) -> BigInt {
    let mut a = a;
    let mut b = b;
    let n = b;
    let ZERO = BigInt::from(0);
    let ONE = BigInt::from(1);

    let mut q = ZERO;
    let mut r = ONE;
    let mut s1 = ONE;
    let mut s2 = ZERO;
    let mut s3 = ONE;
    let mut t1 = ZERO;
    let mut t2 = ONE;
    let mut t3 = ZERO;

    while r > ZERO {
        q = b / a;
        r = b - q * a;
        s3 = s1 - q * s2;
        t3 = t1 - q * t2;

        if r > ZERO {
            b = a;
            a = r;
            s1 = s2;
            s2 = s3;
            t1 = t2;
            t2 = t3;
        }
    }
    (t2 + n).rem(n)
}

fn prime_factors(a: BigInt) -> Vec<BigInt> {
    let mut ans: Vec<BigInt> = Vec::new();
    let mut x = BigInt::from(2);
    while x <= a.sqrt() {
        if a.rem(x) == 0 {
            ans.push(x);
        }
        x += 1;
    }
    ans
}

fn is_primitive_root(a: BigInt, deg: BigInt, N: BigInt) -> bool {
    a.mod_exp(deg, N) == 1
        && prime_factors(deg)
            .iter()
            .map(|&x| a.mod_exp(deg / x, N) != 1)
            .all(|x| x)
}

pub fn working_modulus(n: BigInt, M: BigInt) -> Constants {
    let mut N = n + 1;
    let mut k = BigInt::from(1);
    while !(is_prime(N) && N >= M) {
        k += 1;
        N = k * n + 1;
    }
    assert!(N >= M);
    let mut gen = BigInt::from(0);
    let ONE = BigInt::from(1);
    let mut g = BigInt::from(2);
    while g < N {
        if is_primitive_root(g, N - 1, N) {
            gen = g;
            break;
        }
        g += ONE;
    }
    assert!(gen > 0);
    let w = gen.mod_exp(k, N);
    Constants { N, w }
}

fn order_reverse(inp: &mut Vec<BigInt>) {
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

fn fft(inp: Vec<BigInt>, c: &Constants, w: BigInt) -> Vec<BigInt> {
    let mut inp = inp.clone();
    let N = inp.len();
    let MOD = BigInt::from(c.N);
    let ONE = BigInt::from(1);
    let mut pre: Vec<BigInt> = vec![ONE; N / 2];
    let CHUNK_COUNT = 128;
    let chunk_count = BigInt::from(CHUNK_COUNT);

    pre.par_chunks_mut(CHUNK_COUNT)
        .enumerate()
        .for_each(|(i, arr)| arr[0] = w.mod_exp(BigInt::from(i) * chunk_count, MOD));
    pre.par_chunks_mut(CHUNK_COUNT).for_each(|x| {
        (1..x.len()).for_each(|y| {
            let _x = x.to_vec();
            x[y] = (w * x[y - 1]).rem(MOD);
        })
    });
    order_reverse(&mut inp);

    let mut gap = inp.len() / 2;

    while gap > 0 {
        let nchunks = inp.len() / (2 * gap);
        inp.par_chunks_mut(2 * gap).for_each(|cxi| {
            let (lo, hi) = cxi.split_at_mut(gap);
            lo.par_iter_mut()
                .zip(hi)
                .enumerate()
                .for_each(|(idx, (lo, hi))| {
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
                    *hi = neg.mul_mod(pre[nchunks * idx], MOD);
                });
        });
        gap /= 2;
    }
    inp
}

pub fn forward(inp: Vec<BigInt>, c: &Constants) -> Vec<BigInt> {
    fft(inp, c, c.w)
}

pub fn inverse(inp: Vec<BigInt>, c: &Constants) -> Vec<BigInt> {
    let inv = extended_gcd(BigInt::from(inp.len()), BigInt::from(c.N));
    let w = extended_gcd(c.w, c.N);

    let mut res = fft(inp, c, w);
    res.par_iter_mut().for_each(|x| *x = (inv * (*x)).rem(c.N));
    res
}

#[cfg(test)]
mod tests {
    use rand::Rng;
    use rayon::{iter::ParallelIterator, slice::ParallelSliceMut};

    use crate::{
        ntt::{extended_gcd, forward, inverse, working_modulus},
        numbers::BigInt,
    };

    #[test]
    fn test_forward() {
        (0..100).for_each(|_| {
            let n = 1 << rand::thread_rng().gen::<u32>() % 8;
            let v: Vec<BigInt> = (0..n)
                .map(|_| BigInt::from(rand::thread_rng().gen::<u32>() % (1 << 6)))
                .collect();
            let M = (*v.iter().max().unwrap() << 1) * BigInt::from(n) + 1;
            let c = working_modulus(BigInt::from(n), BigInt::from(M));
            let forward = forward(v.clone(), &c);
            let inverse = inverse(forward, &c);
            v.iter().zip(inverse).for_each(|(&a, b)| assert_eq!(a, b));
        })
    }

    #[test]
    fn test_extended_gcd() {
        (2..11).for_each(|x: u64| {
            let inv = extended_gcd(BigInt::from(x), BigInt::from(11));
            assert_eq!(
                (BigInt::from(x) * inv).rem(BigInt::from(11)),
                BigInt::from(1)
            );
        });
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
