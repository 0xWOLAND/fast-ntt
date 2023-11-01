use mod_exp::mod_exp;

use crate::numbers::BigInt;

fn check_composite(n: BigInt, a: BigInt, d: BigInt, s: BigInt) -> bool {
    let mut x = a.mod_exp(d, n);
    if x == 1 || x == n - 1 {
        return false;
    }
    let mut i = BigInt::from(1);
    while i < s {
        x = (x * x).rem(n);
        if x == n - 1 {
            return false;
        }
        i += 1;
    }
    return true;
}

pub fn is_prime(n: BigInt) -> bool {
    if n < 4 {
        return n == 2 || n == 3;
    }
    let mut s = BigInt::from(0);
    let mut d: BigInt = n - 1;
    while d.rem(BigInt::from(2)) == 0 {
        d >>= 1;
        s += 1;
    }

    for _ in 0..5 {
        if check_composite(n, BigInt::from(2), d, s) {
            return false;
        }
    }
    true
}

#[cfg(test)]
pub mod tests {
    use crate::{numbers::BigInt, prime::is_prime};

    #[test]
    pub fn test_is_prime() {
        assert!(is_prime(BigInt::from(11)));
        assert!(!is_prime(BigInt::from(10)));
    }
}
