use std::{
    fmt::Display,
    num::NonZeroU128,
    ops::{
        Add, AddAssign, BitAnd, BitOr, Div, DivAssign, Mul, MulAssign, Neg, Shl, ShlAssign, Shr,
        ShrAssign, Sub, SubAssign,
    },
};

use crypto_bigint::{rand_core::OsRng, Invert, NonZero, Random, RandomMod, Wrapping, U256};
use itertools::Itertools;

pub enum BigIntType {
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
}

#[derive(Debug, Clone, Copy)]
pub struct BigInt {
    pub v: U256,
}

impl BigInt {
    pub fn new(_v: BigIntType) -> Self {
        Self {
            v: match _v {
                BigIntType::U16(x) => U256::from(x),
                BigIntType::U32(x) => U256::from(x),
                BigIntType::U64(x) => U256::from(x),
                BigIntType::U128(x) => U256::from(x),
                _ => panic!("received invalid `BigIntType`"),
            },
        }
    }

    pub fn rem(&self, num: BigInt) -> Self {
        BigInt {
            v: self.v.const_rem(&num.v).0,
        }
    }

    pub fn mod_exp(&self, exp: BigInt, M: BigInt) -> BigInt {
        let mut res: BigInt = if exp & 1 > 0 {
            self.clone()
        } else {
            BigInt::from(1)
        };
        let mut b = self.clone();
        let mut e = exp.clone();
        while e > 0 {
            e >>= 1;
            b = (b * b).rem(M);
            if e & 1 > 0 {
                res = (res * b).rem(M);
            }
        }
        res
    }

    pub fn sqrt(&self) -> BigInt {
        BigInt {
            v: self.v.sqrt_vartime(),
        }
    }

    pub fn add_mod(&self, rhs: BigInt, M: BigInt) -> BigInt {
        (*self + rhs).rem(M)
    }

    pub fn mul_mod(&self, rhs: BigInt, M: BigInt) -> BigInt {
        (*self * rhs).rem(M)
    }

    pub fn sub_mod(&self, rhs: BigInt, M: BigInt) -> BigInt {
        if rhs > *self {
            M - (rhs - *self).rem(M)
        } else {
            (*self - rhs).rem(M)
        }
    }

    pub fn random() -> BigInt {
        BigInt {
            v: U256::random(&mut OsRng),
        }
    }

    pub fn reverse(&self) -> BigInt {
        let mut v = self.v;
        BigInt { v }
    }
}

impl From<u16> for BigInt {
    fn from(value: u16) -> Self {
        BigInt::new(BigIntType::U16(value))
    }
}

impl From<i32> for BigInt {
    fn from(value: i32) -> Self {
        BigInt::new(BigIntType::U32(value as u32))
    }
}

impl From<usize> for BigInt {
    fn from(value: usize) -> Self {
        BigInt::new(BigIntType::U64(value as u64))
    }
}

impl From<u32> for BigInt {
    fn from(value: u32) -> Self {
        BigInt::new(BigIntType::U32(value))
    }
}

impl From<u64> for BigInt {
    fn from(value: u64) -> Self {
        BigInt::new(BigIntType::U64(value))
    }
}

impl From<u128> for BigInt {
    fn from(value: u128) -> Self {
        BigInt::new(BigIntType::U128(value))
    }
}

impl Add for BigInt {
    type Output = BigInt;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            v: (Wrapping(self.v) + Wrapping(rhs.v)).0,
        }
    }
}

impl Add<u16> for BigInt {
    type Output = BigInt;

    fn add(self, rhs: u16) -> Self::Output {
        Self {
            v: (Wrapping(self.v) + Wrapping(BigInt::from(rhs).v)).0,
        }
    }
}

impl Add<i32> for BigInt {
    type Output = BigInt;

    fn add(self, rhs: i32) -> Self::Output {
        Self {
            v: (Wrapping(self.v) + Wrapping(BigInt::from(rhs).v)).0,
        }
    }
}

impl Add<u32> for BigInt {
    type Output = BigInt;

    fn add(self, rhs: u32) -> Self::Output {
        Self {
            v: (-Wrapping(BigInt::from(rhs).v)).0,
        }
    }
}

impl Add<u64> for BigInt {
    type Output = BigInt;

    fn add(self, rhs: u64) -> Self::Output {
        Self {
            v: (Wrapping(self.v) + Wrapping(BigInt::from(rhs).v)).0,
        }
    }
}

impl Add<u128> for BigInt {
    type Output = BigInt;

    fn add(self, rhs: u128) -> Self::Output {
        Self {
            v: (Wrapping(self.v) + Wrapping(BigInt::from(rhs).v)).0,
        }
    }
}

impl AddAssign for BigInt {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl AddAssign<u16> for BigInt {
    fn add_assign(&mut self, rhs: u16) {
        *self = *self + BigInt::from(rhs);
    }
}

impl AddAssign<u32> for BigInt {
    fn add_assign(&mut self, rhs: u32) {
        *self = *self + BigInt::from(rhs);
    }
}

impl AddAssign<i32> for BigInt {
    fn add_assign(&mut self, rhs: i32) {
        *self = *self + BigInt::from(rhs);
    }
}

impl AddAssign<u64> for BigInt {
    fn add_assign(&mut self, rhs: u64) {
        *self = *self + BigInt::from(rhs);
    }
}

impl AddAssign<u128> for BigInt {
    fn add_assign(&mut self, rhs: u128) {
        *self = *self + BigInt::from(rhs);
    }
}

impl Sub for BigInt {
    type Output = BigInt;

    fn sub(self, rhs: Self) -> Self::Output {
        BigInt {
            v: (Wrapping(self.v) - Wrapping(rhs.v)).0,
        }
    }
}

impl Sub<usize> for BigInt {
    type Output = BigInt;

    fn sub(self, rhs: usize) -> Self::Output {
        self - BigInt::from(rhs)
    }
}

impl Sub<u16> for BigInt {
    type Output = BigInt;

    fn sub(self, rhs: u16) -> Self::Output {
        self - BigInt::from(rhs)
    }
}

impl Sub<u32> for BigInt {
    type Output = BigInt;

    fn sub(self, rhs: u32) -> Self::Output {
        self - BigInt::from(rhs)
    }
}

impl Sub<i32> for BigInt {
    type Output = BigInt;

    fn sub(self, rhs: i32) -> Self::Output {
        self - BigInt::from(rhs)
    }
}

impl Sub<u64> for BigInt {
    type Output = BigInt;

    fn sub(self, rhs: u64) -> Self::Output {
        self - BigInt::from(rhs)
    }
}

impl Sub<u128> for BigInt {
    type Output = BigInt;

    fn sub(self, rhs: u128) -> Self::Output {
        self - BigInt::from(rhs)
    }
}

impl SubAssign for BigInt {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}

impl SubAssign<u16> for BigInt {
    fn sub_assign(&mut self, rhs: u16) {
        *self = *self - BigInt::from(rhs);
    }
}

impl SubAssign<u32> for BigInt {
    fn sub_assign(&mut self, rhs: u32) {
        *self = *self - BigInt::from(rhs);
    }
}

impl SubAssign<i32> for BigInt {
    fn sub_assign(&mut self, rhs: i32) {
        *self = *self - BigInt::from(rhs);
    }
}

impl SubAssign<u64> for BigInt {
    fn sub_assign(&mut self, rhs: u64) {
        *self = *self - BigInt::from(rhs);
    }
}

impl SubAssign<u128> for BigInt {
    fn sub_assign(&mut self, rhs: u128) {
        *self = *self - BigInt::from(rhs);
    }
}

impl Neg for BigInt {
    type Output = BigInt;

    fn neg(self) -> Self::Output {
        Self {
            v: (Wrapping(U256::MAX) - Wrapping(self.v)).0,
        }
    }
}

impl Mul for BigInt {
    type Output = BigInt;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            v: (Wrapping(self.v) * Wrapping(rhs.v)).0,
        }
    }
}

impl MulAssign for BigInt {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs
    }
}

impl Div for BigInt {
    type Output = BigInt;

    fn div(self, rhs: Self) -> Self::Output {
        let [lower, upper, _, _] = rhs.v.to_words();
        let half = (upper as u128) << 64;
        let half = half + (lower as u128);
        BigInt {
            v: (Wrapping(self.v) / NonZero::from(NonZeroU128::new(half).unwrap())).0,
        }
    }
}

impl Invert for BigInt {
    type Output = BigInt;

    fn invert(&self) -> Self::Output {
        BigInt {
            v: self.v.inv_mod(&U256::MAX).0,
        }
    }
}

impl DivAssign for BigInt {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl Eq for BigInt {}

impl Ord for BigInt {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.v.cmp(&other.v)
    }
}

impl PartialEq for BigInt {
    fn eq(&self, other: &Self) -> bool {
        self.v == other.v
    }
}

impl PartialEq<u16> for BigInt {
    fn eq(&self, other: &u16) -> bool {
        self.v == BigInt::from(*other).v
    }
}

impl PartialEq<i32> for BigInt {
    fn eq(&self, other: &i32) -> bool {
        self.v == BigInt::from(*other).v
    }
}

impl PartialEq<u32> for BigInt {
    fn eq(&self, other: &u32) -> bool {
        self.v == BigInt::from(*other).v
    }
}

impl PartialEq<u64> for BigInt {
    fn eq(&self, other: &u64) -> bool {
        self.v == BigInt::from(*other).v
    }
}

impl PartialEq<u128> for BigInt {
    fn eq(&self, other: &u128) -> bool {
        self.v == BigInt::from(*other).v
    }
}

impl PartialOrd for BigInt {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.v.partial_cmp(&other.v)
    }
}

impl PartialOrd<u16> for BigInt {
    fn partial_cmp(&self, other: &u16) -> Option<std::cmp::Ordering> {
        self.v.partial_cmp(&BigInt::from(*other).v)
    }
}

impl PartialOrd<i32> for BigInt {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        self.v.partial_cmp(&BigInt::from(*other).v)
    }
}

impl PartialOrd<u32> for BigInt {
    fn partial_cmp(&self, other: &u32) -> Option<std::cmp::Ordering> {
        self.v.partial_cmp(&BigInt::from(*other).v)
    }
}

impl PartialOrd<u64> for BigInt {
    fn partial_cmp(&self, other: &u64) -> Option<std::cmp::Ordering> {
        self.v.partial_cmp(&BigInt::from(*other).v)
    }
}

impl PartialOrd<u128> for BigInt {
    fn partial_cmp(&self, other: &u128) -> Option<std::cmp::Ordering> {
        self.v.partial_cmp(&BigInt::from(*other).v)
    }
}

impl BitAnd for BigInt {
    type Output = BigInt;

    fn bitand(self, rhs: Self) -> Self::Output {
        BigInt { v: self.v & rhs.v }
    }
}

impl BitAnd<u16> for BigInt {
    type Output = BigInt;

    fn bitand(self, rhs: u16) -> Self::Output {
        BigInt {
            v: self.v & BigInt::from(rhs).v,
        }
    }
}

impl BitAnd<i32> for BigInt {
    type Output = BigInt;

    fn bitand(self, rhs: i32) -> Self::Output {
        BigInt {
            v: self.v & BigInt::from(rhs).v,
        }
    }
}

impl BitAnd<u32> for BigInt {
    type Output = BigInt;

    fn bitand(self, rhs: u32) -> Self::Output {
        BigInt {
            v: self.v & BigInt::from(rhs).v,
        }
    }
}

impl BitAnd<u64> for BigInt {
    type Output = BigInt;

    fn bitand(self, rhs: u64) -> Self::Output {
        BigInt {
            v: self.v & BigInt::from(rhs).v,
        }
    }
}

impl BitAnd<u128> for BigInt {
    type Output = BigInt;

    fn bitand(self, rhs: u128) -> Self::Output {
        BigInt {
            v: self.v & BigInt::from(rhs).v,
        }
    }
}

impl BitOr for BigInt {
    type Output = BigInt;

    fn bitor(self, rhs: Self) -> Self::Output {
        BigInt { v: self.v | rhs.v }
    }
}

impl BitOr<u16> for BigInt {
    type Output = BigInt;

    fn bitor(self, rhs: u16) -> Self::Output {
        BigInt {
            v: self.v | BigInt::from(rhs).v,
        }
    }
}

impl BitOr<i32> for BigInt {
    type Output = BigInt;

    fn bitor(self, rhs: i32) -> Self::Output {
        BigInt {
            v: self.v | BigInt::from(rhs).v,
        }
    }
}

impl BitOr<u32> for BigInt {
    type Output = BigInt;

    fn bitor(self, rhs: u32) -> Self::Output {
        BigInt {
            v: self.v | BigInt::from(rhs).v,
        }
    }
}

impl BitOr<u64> for BigInt {
    type Output = BigInt;

    fn bitor(self, rhs: u64) -> Self::Output {
        BigInt {
            v: self.v | BigInt::from(rhs).v,
        }
    }
}

impl BitOr<u128> for BigInt {
    type Output = BigInt;

    fn bitor(self, rhs: u128) -> Self::Output {
        BigInt {
            v: self.v | BigInt::from(rhs).v,
        }
    }
}

impl Shl<usize> for BigInt {
    type Output = BigInt;

    fn shl(self, rhs: usize) -> Self::Output {
        BigInt { v: self.v << rhs }
    }
}

impl Shr<usize> for BigInt {
    type Output = BigInt;

    fn shr(self, rhs: usize) -> Self::Output {
        BigInt { v: self.v >> rhs }
    }
}

impl ShrAssign<usize> for BigInt {
    fn shr_assign(&mut self, rhs: usize) {
        *self = *self >> rhs;
    }
}

impl ShlAssign<usize> for BigInt {
    fn shl_assign(&mut self, rhs: usize) {
        *self = *self << rhs;
    }
}

impl Display for BigInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // concatenate bytes to string representation
        let str: String = self
            .v
            .to_words()
            .iter()
            .rev()
            .map(|&x| x.to_string())
            .join("");

        let mut trimmed = str.trim_start_matches('0').to_string();

        // in case the string is "0000"
        if str.chars().all(|x| x == '0') {
            trimmed = "0".to_string();
        }

        write!(f, "{}", trimmed)
    }
}

#[cfg(test)]
mod tests {
    use crate::numbers::BigInt;
    use mod_exp::mod_exp;

    #[test]
    fn test_mod_exp() {
        let N = 73;
        (2..10).for_each(|x| {
            (2..10).for_each(|y| {
                assert_eq!(
                    BigInt::from(mod_exp(x, y, N)),
                    BigInt::from(x).mod_exp(BigInt::from(y), BigInt::from(N))
                );
            })
        })
    }

    #[test]
    fn test_division() {
        let a = BigInt::from(8);
        let b = BigInt::from(10);
        println!("{}", a / b);
    }

    #[test]
    fn test_rem() {
        let a = BigInt::from(10);
        println!("{}", a.rem(BigInt::from(4)));
    }

    #[test]
    fn test_display() {
        let a = BigInt::from(111);
        println!("{}", a);
    }

    #[test]
    fn test_add_mod() {
        let a = BigInt::from(72);
        let b = BigInt::from(1890);
        let N = BigInt::from(73);

        println!("{}", a.sub_mod(b, N));
    }
}
