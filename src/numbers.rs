use std::{
    cmp::Ordering,
    fmt::Display,
    num::NonZeroU128,
    ops::{
        Add, AddAssign, BitAnd, BitOr, Div, DivAssign, Mul, MulAssign, Neg, Shl, ShlAssign, Shr,
        ShrAssign, Sub, SubAssign,
    },
};

use crypto_bigint::{
    modular::{
        runtime_mod::{DynResidue, DynResidueParams},
        Retrieve,
    },
    Invert, NonZero, Uint, U128, U256,
};
use itertools::Itertools;
use rand::{thread_rng, Error, Rng};

use crate::polynomial::PolynomialFieldElement;

pub enum BigIntType {
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
}

pub trait NttFieldElement {
    // all operations should be under the modular group `M`
    fn set_mod(&mut self, M: Self) -> Result<(), String>;
    fn rem(&self, M: Self) -> Self;
    fn pow(&self, n: u128) -> Self;
    fn mod_exp(&self, exp: Self, M: Self) -> Self;
    fn is_even(&self) -> bool;
    fn is_zero(&self) -> bool;
    fn to_bigint(&self) -> BigInt;
}

#[derive(Debug, Clone, Copy)]
pub struct BigInt {
    pub v: DynResidue<4>,
}

impl BigInt {
    pub fn new(_v: BigIntType) -> Self {
        let params = DynResidueParams::new(&U256::from_be_hex(
            "ffffffff00000000ffffffffffffffffbce6faada7179e84f3b9cac2fc632551",
        ));
        Self {
            v: match _v {
                BigIntType::U16(x) => DynResidue::new(&U256::from(x), params),
                BigIntType::U32(x) => DynResidue::new(&U256::from(x), params),
                BigIntType::U64(x) => DynResidue::new(&U256::from(x), params),
                BigIntType::U128(x) => DynResidue::new(&U256::from(x), params),
                _ => panic!("received invalid `BigIntType`"),
            },
        }
    }

    pub fn set_mod(&mut self, M: BigInt) -> Result<(), String> {
        if M.is_even() {
            return Err("modulus must be odd".to_string());
        }
        let params = DynResidueParams::new(&(U256::from(M.v.retrieve())));
        self.v = DynResidue::new(&self.v.retrieve(), params);
        Ok(())
    }

    pub fn set_mod_from_residue(&mut self, params: DynResidueParams<4>) {
        self.v = DynResidue::new(&self.v.retrieve(), params);
    }

    pub fn rem(&self, M: BigInt) -> BigInt {
        let mut res = self.clone();
        if res < M {
            return res;
        }
        res.v = DynResidue::new(
            &res.v.retrieve().rem(&NonZero::from_uint(M.v.retrieve())),
            res.params(),
        );
        res
    }

    pub fn params(&self) -> DynResidueParams<4> {
        *self.v.params()
    }

    pub fn pow(&self, n: u128) -> BigInt {
        BigInt {
            v: self.v.pow(&Uint::<4>::from_u128(n)),
        }
    }

    pub fn mod_exp(&self, exp: BigInt, M: BigInt) -> BigInt {
        let mut res: BigInt = if !exp.is_even() {
            self.clone()
        } else {
            BigInt::from(1)
        };
        let mut b = self.clone();
        let mut e = exp.clone();
        res.set_mod(M);
        b.set_mod(M);
        while e > 0 {
            e >>= 1;
            b = b * b;
            if M.is_even() {
                b = b.rem(M);
            }
            if !e.is_even() && !e.is_zero() {
                res = b * res;
                if M.is_even() {
                    res = res.rem(M);
                }
            }
        }
        res
    }

    pub fn random() -> BigInt {
        let x = rand::thread_rng().gen::<u128>();
        BigInt::new(BigIntType::U128(x))
    }

    pub fn is_zero(&self) -> bool {
        self.v.retrieve().bits() == 0
    }

    pub fn is_even(&self) -> bool {
        let is_odd: bool = self.v.retrieve().bit(0).into();
        !is_odd
    }

    pub fn to_u32(&self) -> Result<u32, String> {
        let ret = self.v.retrieve().as_words()[0] as u32;
        if BigInt::from(ret) != *self {
            return Err(format!("Overflow error -- {} exceeds u32 size limits", self).to_string());
        }
        Ok(ret)
    }
}

impl NttFieldElement for BigInt {
    fn set_mod(&mut self, M: Self) -> Result<(), String> {
        if M.is_even() {
            return Err("modulus must be odd".to_string());
        }
        let params = DynResidueParams::new(&(U256::from(M.v.retrieve())));
        self.v = DynResidue::new(&self.v.retrieve(), params);
        Ok(())
    }

    fn rem(&self, M: Self) -> BigInt {
        let mut res = self.clone();
        if res < M {
            return res;
        }
        res.v = DynResidue::new(
            &res.v.retrieve().rem(&NonZero::from_uint(M.v.retrieve())),
            res.params(),
        );
        res
    }

    fn pow(&self, n: u128) -> BigInt {
        BigInt {
            v: self.v.pow(&Uint::<4>::from_u128(n)),
        }
    }

    fn mod_exp(&self, exp: BigInt, M: BigInt) -> BigInt {
        let mut res: BigInt = if !exp.is_even() {
            self.clone()
        } else {
            BigInt::from(1)
        };
        let mut b = self.clone();
        let mut e = exp.clone();
        res.set_mod(M);
        b.set_mod(M);
        while e > 0 {
            e >>= 1;
            b = b * b;
            if M.is_even() {
                b = b.rem(M);
            }
            if !e.is_even() && !e.is_zero() {
                res = b * res;
                if M.is_even() {
                    res = res.rem(M);
                }
            }
        }
        res
    }

    fn is_zero(&self) -> bool {
        self.v.retrieve().bits() == 0
    }

    fn is_even(&self) -> bool {
        let is_odd: bool = self.v.retrieve().bit(0).into();
        !is_odd
    }

    fn to_bigint(&self) -> BigInt {
        *self
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
        if rhs.v.params() != self.v.params() {
            let mut rhs = rhs.clone();
            rhs.set_mod_from_residue(self.params());
            return Self { v: self.v + rhs.v };
        }
        Self { v: self.v + rhs.v }
    }
}

impl Add<u16> for BigInt {
    type Output = BigInt;

    fn add(self, rhs: u16) -> Self::Output {
        Self {
            v: self.v + BigInt::from(rhs).v,
        }
    }
}

impl Add<i32> for BigInt {
    type Output = BigInt;

    fn add(self, rhs: i32) -> Self::Output {
        Self {
            v: self.v + BigInt::from(rhs).v,
        }
    }
}

impl Add<u32> for BigInt {
    type Output = BigInt;

    fn add(self, rhs: u32) -> Self::Output {
        Self {
            v: self.v + BigInt::from(rhs).v,
        }
    }
}

impl Add<u64> for BigInt {
    type Output = BigInt;

    fn add(self, rhs: u64) -> Self::Output {
        Self {
            v: self.v + BigInt::from(rhs).v,
        }
    }
}

impl Add<u128> for BigInt {
    type Output = BigInt;

    fn add(self, rhs: u128) -> Self::Output {
        Self {
            v: self.v + BigInt::from(rhs).v,
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
        if rhs.v.params() != self.v.params() {
            let mut rhs = rhs.clone();
            rhs.set_mod_from_residue(self.params());
            return Self { v: self.v - rhs.v };
        }
        Self { v: self.v - rhs.v }
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
        Self { v: self.v.neg() }
    }
}

impl Mul for BigInt {
    type Output = BigInt;

    fn mul(self, rhs: Self) -> Self::Output {
        if rhs.v.params() != self.v.params() {
            let mut rhs = rhs.clone();
            rhs.set_mod_from_residue(self.params());
            return Self { v: self.v * rhs.v };
        }
        Self { v: self.v * rhs.v }
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
        BigInt {
            v: DynResidue::new(
                &(self
                    .v
                    .retrieve()
                    .div_rem(&NonZero::from_uint(rhs.v.retrieve()))
                    .0),
                self.params(),
            ),
        }
    }
}

impl Invert for BigInt {
    type Output = BigInt;

    fn invert(&self) -> Self::Output {
        BigInt {
            v: self.v.invert().0,
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
        let half = self
            .params()
            .modulus()
            .div(NonZero::from(NonZeroU128::new(2).unwrap()));
        (self.v - other.v).retrieve().cmp(&half)
    }
}

impl PartialEq for BigInt {
    fn eq(&self, other: &Self) -> bool {
        self.v.retrieve() == other.v.retrieve()
    }
}

impl PartialEq<u16> for BigInt {
    fn eq(&self, other: &u16) -> bool {
        self.v.retrieve() == BigInt::from(*other).v.retrieve()
    }
}

impl PartialEq<i32> for BigInt {
    fn eq(&self, other: &i32) -> bool {
        self.v.retrieve() == BigInt::from(*other).v.retrieve()
    }
}

impl PartialEq<u32> for BigInt {
    fn eq(&self, other: &u32) -> bool {
        self.v.retrieve() == BigInt::from(*other).v.retrieve()
    }
}

impl PartialEq<u64> for BigInt {
    fn eq(&self, other: &u64) -> bool {
        self.v.retrieve() == BigInt::from(*other).v.retrieve()
    }
}

impl PartialEq<u128> for BigInt {
    fn eq(&self, other: &u128) -> bool {
        self.v.retrieve() == BigInt::from(*other).v.retrieve()
    }
}

impl PartialOrd for BigInt {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (self.v.retrieve()).partial_cmp(&(other.v.retrieve()))
    }
}

impl PartialOrd<u16> for BigInt {
    fn partial_cmp(&self, other: &u16) -> Option<std::cmp::Ordering> {
        (self.v.retrieve()).partial_cmp(&BigInt::from(*other).v.retrieve())
    }
}

impl PartialOrd<i32> for BigInt {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        (self.v.retrieve()).partial_cmp(&BigInt::from(*other).v.retrieve())
    }
}

impl PartialOrd<u32> for BigInt {
    fn partial_cmp(&self, other: &u32) -> Option<std::cmp::Ordering> {
        (self.v.retrieve()).partial_cmp(&BigInt::from(*other).v.retrieve())
    }
}

impl PartialOrd<u64> for BigInt {
    fn partial_cmp(&self, other: &u64) -> Option<std::cmp::Ordering> {
        (self.v.retrieve()).partial_cmp(&BigInt::from(*other).v.retrieve())
    }
}

impl PartialOrd<u128> for BigInt {
    fn partial_cmp(&self, other: &u128) -> Option<std::cmp::Ordering> {
        (self.v.retrieve()).partial_cmp(&BigInt::from(*other).v.retrieve())
    }
}

impl BitAnd for BigInt {
    type Output = BigInt;

    fn bitand(self, rhs: Self) -> Self::Output {
        BigInt {
            v: DynResidue::new(&(self.v.retrieve() & rhs.v.retrieve()), self.params()),
        }
    }
}

impl BitAnd<u16> for BigInt {
    type Output = BigInt;

    fn bitand(self, rhs: u16) -> Self::Output {
        BigInt {
            v: DynResidue::new(
                &(self.v.retrieve() & BigInt::from(rhs).v.retrieve()),
                self.params(),
            ),
        }
    }
}

impl BitAnd<i32> for BigInt {
    type Output = BigInt;

    fn bitand(self, rhs: i32) -> Self::Output {
        BigInt {
            v: DynResidue::new(
                &(self.v.retrieve() & BigInt::from(rhs).v.retrieve()),
                self.params(),
            ),
        }
    }
}

impl BitAnd<u32> for BigInt {
    type Output = BigInt;

    fn bitand(self, rhs: u32) -> Self::Output {
        BigInt {
            v: DynResidue::new(
                &(self.v.retrieve() & BigInt::from(rhs).v.retrieve()),
                self.params(),
            ),
        }
    }
}

impl BitAnd<u64> for BigInt {
    type Output = BigInt;

    fn bitand(self, rhs: u64) -> Self::Output {
        BigInt {
            v: DynResidue::new(
                &(self.v.retrieve() & BigInt::from(rhs).v.retrieve()),
                self.params(),
            ),
        }
    }
}

impl BitAnd<u128> for BigInt {
    type Output = BigInt;

    fn bitand(self, rhs: u128) -> Self::Output {
        BigInt {
            v: DynResidue::new(
                &(self.v.retrieve() & BigInt::from(rhs).v.retrieve()),
                self.params(),
            ),
        }
    }
}

impl BitOr for BigInt {
    type Output = BigInt;

    fn bitor(self, rhs: Self) -> Self::Output {
        BigInt {
            v: DynResidue::new(&(self.v.retrieve() | rhs.v.retrieve()), self.params()),
        }
    }
}

impl BitOr<u16> for BigInt {
    type Output = BigInt;

    fn bitor(self, rhs: u16) -> Self::Output {
        BigInt {
            v: DynResidue::new(
                &(self.v.retrieve() | BigInt::from(rhs).v.retrieve()),
                self.params(),
            ),
        }
    }
}

impl BitOr<i32> for BigInt {
    type Output = BigInt;

    fn bitor(self, rhs: i32) -> Self::Output {
        BigInt {
            v: DynResidue::new(
                &(self.v.retrieve() | BigInt::from(rhs).v.retrieve()),
                self.params(),
            ),
        }
    }
}

impl BitOr<u32> for BigInt {
    type Output = BigInt;

    fn bitor(self, rhs: u32) -> Self::Output {
        BigInt {
            v: DynResidue::new(
                &(self.v.retrieve() | BigInt::from(rhs).v.retrieve()),
                self.params(),
            ),
        }
    }
}

impl BitOr<u64> for BigInt {
    type Output = BigInt;

    fn bitor(self, rhs: u64) -> Self::Output {
        BigInt {
            v: DynResidue::new(
                &(self.v.retrieve() | BigInt::from(rhs).v.retrieve()),
                self.params(),
            ),
        }
    }
}

impl BitOr<u128> for BigInt {
    type Output = BigInt;

    fn bitor(self, rhs: u128) -> Self::Output {
        BigInt {
            v: DynResidue::new(
                &(self.v.retrieve() | BigInt::from(rhs).v.retrieve()),
                self.params(),
            ),
        }
    }
}

impl Shl<usize> for BigInt {
    type Output = BigInt;

    fn shl(self, rhs: usize) -> Self::Output {
        BigInt {
            v: DynResidue::new(&self.v.retrieve().shl_vartime(rhs), self.params()),
        }
    }
}

impl Shr<usize> for BigInt {
    type Output = BigInt;

    fn shr(self, rhs: usize) -> Self::Output {
        BigInt {
            v: DynResidue::new(&self.v.retrieve().shr_vartime(rhs), self.params()),
        }
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
            .retrieve()
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

impl PolynomialFieldElement for BigInt {}

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
        });
    }

    #[test]
    fn test_mul() {
        let a = BigInt::from(8);
        let b = BigInt::from(10);
        println!("{}", a * b);
    }

    #[test]
    fn test_is_even() {
        let a = BigInt::from(1 << 12);
        assert!(a.is_even());
    }

    #[test]
    fn test_division() {
        let a = BigInt::from(8);
        let b = BigInt::from(10);
        assert_eq!(a / b, BigInt::from(0))
    }

    #[test]
    fn test_rem() {
        let a = BigInt::from(10);
        assert_eq!(a.rem(BigInt::from(4)), BigInt::from(2));
    }

    #[test]
    fn test_display() {
        let a = BigInt::from(111);
        println!("{}", a);
    }

    #[test]
    fn test_shr() {
        let a = BigInt::from(1);
        println!("{}", a >> 1);
    }
}
