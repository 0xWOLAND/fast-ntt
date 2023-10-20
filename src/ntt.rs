use crate::prime::is_prime;
use mod_exp::mod_exp;
use std::mem::swap;

pub struct Constants {
    k: i64,
    N: i64,
    w: i64,
}

fn gcd(a: i64, b: i64) -> i64 {
    let mut a = a;
    let mut b = b;

    if a < b {
        swap(&mut a, &mut b);
    }
    if a % b == 0 {
        return b;
    }
    while b > 0 {
        a = a % b;
        swap(&mut a, &mut b);
    }
    a
}

fn extended_gcd(a: i64, b: i64) -> i64 {
    let mut a = a;
    let mut b = b;
    let n = b;
    let mut q = 0;
    let mut r = 1;
    let mut s1 = 1;
    let mut s2 = 0;
    let mut s3 = 1;
    let mut t1 = 0;
    let mut t2 = 1;
    let mut t3 = 0;

    while r > 0 {
        q = b / a;
        r = b - q * a;
        s3 = s1 - q * s2;
        t3 = t1 - q * t2;

        if r > 0 {
            b = a;
            a = r;
            s1 = s2;
            s2 = s3;
            t1 = t2;
            t2 = t3;
        }
    }
    (t2 + n) % n
}

fn working_modulus(n: i64, M: i64) -> Constants {
    let mut N = n + 1;
    let mut k = 1;
    while (!is_prime(N)) || N < M {
        k += 1;
        N = k * n + 1;
    }
    let mut gen = 0;
    for g in 2..N {
        if gcd(g, N) == 1 {
            gen = g;
            break;
        }
    }
    assert!(gen > 0);
    let w = mod_exp(gen, k, N);
    Constants { k, N, w }
}

pub fn forward(inp: Vec<i64>, c: &Constants) -> Vec<i64> {
    (0..inp.len())
        .map(|k| {
            inp.iter().enumerate().fold(0, |acc, (i, cur)| {
                (acc + cur * mod_exp(c.w, (k * i) as i64, c.N) as i64) % c.N as i64
            }) % c.N as i64
        })
        .collect()
}

pub fn inverse(inp: Vec<i64>, c: &Constants) -> Vec<i64> {
    let inv = extended_gcd(inp.len() as i64, c.N);
    let w = extended_gcd(c.w, c.N);

    (0..inp.len())
        .map(|k| {
            inv as i64
                * inp.iter().enumerate().fold(0, |acc, (i, cur)| {
                    (acc + cur * mod_exp(w, (k * i) as i64, c.N) as i64) % c.N as i64
                })
                % c.N as i64
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::ntt::{extended_gcd, forward, gcd, inverse, working_modulus, Constants};
    #[test]
    fn test_forward() {
        let v: Vec<i64> = vec![6, 0, 10, 7, 2];
        let n = v.len() as i64;
        let M = v.iter().max().unwrap().pow(2) as i64 * n + 1;
        let c = working_modulus(n, M);
        let forward = forward(v.clone(), &c);
        let inverse = inverse(forward, &c);
        v.iter().zip(inverse).for_each(|(&a, b)| assert_eq!(a, b));
    }
    #[test]
    fn test_extended_gcd() {
        (2..11).for_each(|x| {
            let inv = extended_gcd(x, 11);
            println!("{} {}", x, inv);
            assert_eq!((x * inv) % 11, 1);
        });
    }
    #[test]
    fn test_gcd() {
        assert_eq!(gcd(10, 5), 5);
        assert_eq!(gcd(10, 7), 1);
    }
}
