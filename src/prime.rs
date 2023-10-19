use mod_exp::mod_exp;

fn check_composite(n: u64, a: u64, d: u64, s: u64) -> bool {
    let mut x = mod_exp(a, d, n);
    if x == 1 || x == n - 1 {
        return false;
    }
    for _ in 1..s {
        x = x * x % n;
        if x == n - 1 {
            return false;
        }
    }
    return true;
}

pub fn is_prime(n: u64) -> bool {
    if n < 4 {
        return n == 2 || n == 3;
    }
    let mut s = 0;
    let mut d = n - 1;
    while (d & 1) == 0 {
        d >>= 1;
        s += 1;
    }

    for _ in 0..5 {
        if check_composite(n, 2, d, s) {
            return false;
        }
    }
    true
}

#[cfg(test)]
pub mod tests {
    use crate::prime::is_prime;

    #[test]
    pub fn test_is_prime() {
        assert!(is_prime(11));
        assert!(!is_prime(10));
    }
}
