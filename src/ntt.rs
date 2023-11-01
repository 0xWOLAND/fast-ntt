use crate::{numbers::BigInt, prime::is_prime};
use itertools::Itertools;
use mod_exp::mod_exp;

#[derive(Debug, Clone)]
pub struct Constants {
    pub k: BigInt,
    pub N: BigInt,
    pub w: BigInt,
}

fn extended_gcd(a: BigInt, b: BigInt) -> BigInt {
    let mut a = a;
    let mut b = b;
    let n = b;
    let mut q = BigInt::from(0);
    let mut r = BigInt::from(1);
    let mut s1 = BigInt::from(1);
    let mut s2 = BigInt::from(0);
    let mut s3 = BigInt::from(1);
    let mut t1 = BigInt::from(0);
    let mut t2 = BigInt::from(1);
    let mut t3 = BigInt::from(0);

    while r > BigInt::from(0) {
        q = b / a;
        r = b - q * a;
        s3 = s1 - q * s2;
        t3 = t1 - q * t2;

        if r > BigInt::from(0) {
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
    let lhs = a.mod_exp(deg, N) == 1;
    let pf = prime_factors(deg);
    let rhs: Vec<bool> = pf.iter().map(|&x| a.mod_exp(deg / x, N) != 1).collect();
    // .all(|x| x);
    lhs && rhs.iter().all(|&x| x)
}

pub fn working_modulus(n: BigInt, M: BigInt) -> Constants {
    let mut N = n + 1;
    let mut k = BigInt::from(1);
    while (!is_prime(N)) || N < M {
        k += 1;
        N = k * n + 1;
    }
    let mut gen = BigInt::from(0);
    let mut g = BigInt::from(2);
    while g < N {
        if is_primitive_root(g, N - 1, N) {
            gen = g;
            break;
        }
        g += BigInt::from(1);
    }
    assert!(gen > 0);
    let w = gen.mod_exp(k, N);
    Constants { k, N, w }
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
    let mut pre: Vec<BigInt> = vec![BigInt::from(1); N / 2];

    (1..N / 2).for_each(|i| pre[i] = (pre[i - 1] * w).rem(BigInt::from(c.N)));
    order_reverse(&mut inp);

    let mut len = 2;

    while len <= N {
        let half = len / 2;
        let pre_step = N / len;
        (0..N).step_by(len).for_each(|i| {
            let mut k = 0;
            (i..i + half).for_each(|j| {
                let l = j + half;
                let left = inp[j];
                let right = inp[l] * pre[k];
                inp[j] = (left + right).rem(BigInt::from(c.N));
                inp[l] = (left - right).rem(BigInt::from(c.N));
                k += pre_step;
            })
        });
        len <<= 1;
    }
    inp
}

pub fn forward(inp: Vec<BigInt>, c: &Constants) -> Vec<BigInt> {
    fft(inp, c, c.w)
}

pub fn inverse(inp: Vec<BigInt>, c: &Constants) -> Vec<BigInt> {
    let inv = extended_gcd(BigInt::from(inp.len()), BigInt::from(c.N));
    let w = extended_gcd(c.w, c.N);

    fft(inp, c, w)
        .iter()
        .map(|&x| (inv * x).rem(c.N))
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use rand::Rng;

    use crate::{
        ntt::{extended_gcd, forward, inverse, working_modulus, Constants},
        numbers::BigInt,
    };

    #[test]
    fn test_forward() {
        let n = rand::thread_rng().gen::<i32>().abs() % 10;
        let v: Vec<BigInt> = (0..n)
            .map(|_| BigInt::from(rand::thread_rng().gen::<i32>().abs() % (1 << 6)))
            .collect();
        let M = (*v.iter().max().unwrap() << 1) * BigInt::from(n) + 1;
        let c = working_modulus(BigInt::from(n), BigInt::from(M));
        let forward = forward(v.clone(), &c);
        let inverse = inverse(forward, &c);
        v.iter().zip(inverse).for_each(|(&a, b)| assert_eq!(a, b));
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
}
