use crate::numbers::BigInt;

fn miller_test(mut d: BigInt, n: BigInt, x: BigInt) -> bool {
    let one = BigInt::from(1);
    let two = BigInt::from(2);
    let a = BigInt::from(2) + x;

    let mut x = a.mod_exp(d, n);

    if x == one || x == n - one {
        return true;
    }

    while d != n - one {
        x = (x * x).rem(n);
        d *= two;

        if x == one {
            return false;
        }
        if x == n - one {
            return true;
        }
    }

    false
}

pub fn is_prime(num: BigInt) -> bool {
    let zero = BigInt::from(0);
    let one = BigInt::from(1);
    let two = BigInt::from(2);
    if num <= one || num == BigInt::from(4) {
        return false;
    }
    if num <= BigInt::from(3) {
        return true;
    }

    let mut d = num - one;
    while d.rem(two) == zero {
        d >>= 1;
    }

    for x in 0..4 {
        if miller_test(d, num, BigInt::from(x)) == false {
            return false;
        }
    }
    true
}
#[cfg(test)]
mod tests {
    use crate::{numbers::BigInt, prime::is_prime};

    #[test]
    fn test_is_prime() {
        assert!(is_prime(BigInt::from(11)));
        assert!(!is_prime(BigInt::from(10)));
    }
}
