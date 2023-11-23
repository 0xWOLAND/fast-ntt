use crate::numbers::BigInt;

fn miller_test(mut d: BigInt, n: BigInt, x: BigInt) -> bool {
    let one = BigInt::from(1);
    let two = BigInt::from(2);
    let a = BigInt::from(2) + x;

    let mut x = a.mod_exp(d, n);
    match x.set_mod(n) {
        Ok(()) => (),
        Err(_) => return false,
    };
    match d.set_mod(n) {
        Ok(()) => (),
        Err(_) => return false,
    };

    if x == one || x == n - one {
        return true;
    }

    // (d + 1) mod n = 0
    while !(d + one).is_zero() {
        // x = x * x mod n
        x = x * x;
        d *= two;

        if x == one {
            return false;
        }
        if (x + one).is_zero() {
            return true;
        }
    }

    false
}

pub fn is_prime(num: BigInt) -> bool {
    let one = BigInt::from(1);
    if num <= one || num == BigInt::from(4) {
        return false;
    }
    if num <= BigInt::from(3) {
        return true;
    }

    let mut d = num - one;
    while d.is_even() && !d.is_zero() {
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
