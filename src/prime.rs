use crate::{numbers::BigInt, polynomial::PolynomialFieldElement};

fn miller_test<T: PolynomialFieldElement>(mut d: T, n: T, x: T) -> bool {
    let ONE = T::from(1);
    let TWO = T::from(2);
    let a = TWO + x;

    let mut x = a.mod_exp(d, n);
    match x.set_mod(n) {
        Ok(()) => (),
        Err(_) => return false,
    };
    match d.set_mod(n) {
        Ok(()) => (),
        Err(_) => return false,
    };

    if x == ONE || x == n - ONE {
        return true;
    }

    // (d + 1) mod n = 0
    while !(d + ONE).is_zero() {
        // x = x * x mod n
        x = x * x;
        d *= TWO;

        if x == ONE {
            return false;
        }
        if (x + ONE).is_zero() {
            return true;
        }
    }

    false
}

pub fn is_prime<T: PolynomialFieldElement>(num: T) -> bool {
    let ONE = T::from(1);
    if num <= ONE || num == T::from(4) {
        return false;
    }
    if num <= T::from(3) {
        return true;
    }

    let mut d = num - ONE;
    while d.is_even() && !d.is_zero() {
        d >>= 1;
    }

    for x in 0..4 {
        if miller_test(d, num, T::from(x)) == false {
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
