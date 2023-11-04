use crate::numbers::BigInt;

fn check_composite(n: BigInt, a: BigInt, d: BigInt, s: BigInt) -> bool {
    let ONE = BigInt::from(1);
    let TWO = BigInt::from(2);
    if a.mod_exp(d, n) == ONE {
        return false;
    }

    let mut i = ONE;
    while i < s {
        if a.mod_exp(TWO.mod_exp(i, n), n) == n - ONE {
            return false;
        }
        i += 1;
    }
    true
}

pub fn is_prime(n: BigInt) -> bool {
    if n == 0 || n == 1 || n == 4 || n == 6 || n == 8 || n == 9 {
        return false;
    }
    if n == 2 || n == 3 || n == 5 || n == 7 {
        return true;
    }

    let mut s = BigInt::from(0);
    let mut d: BigInt = n - 1;
    while d.rem(BigInt::from(2)) == 0 {
        d >>= 1;
        s += 1;
    }

    let TWO = BigInt::from(2);

    for _ in 0..8 {
        if check_composite(n, TWO, d, s) {
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
