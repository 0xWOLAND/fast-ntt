use crate::{numbers::BigInt, polynomial::PolynomialFieldElement};

fn miller_test<T: PolynomialFieldElement>(mut d: T, n: T, x: T) -> bool {
    let ONE = T::from_i32(1, BigInt::modulus());
    let TWO = T::from_i32(2, BigInt::modulus());
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
    let ONE = T::from_i32(1, BigInt::modulus());
    if num <= ONE || num == T::from_i32(4, BigInt::modulus()) {
        return false;
    }
    if num <= T::from_i32(3, BigInt::modulus()) {
        return true;
    }

    let mut d = num - ONE;
    while d.is_even() && !d.is_zero() {
        d >>= 1;
    }

    for x in 0..4 {
        if miller_test(d, num, T::from_i32(x, BigInt::modulus())) == false {
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
        assert!(is_prime(BigInt::from_i32(11)));
        assert!(!is_prime(BigInt::from_i32(10)));
    }
}
