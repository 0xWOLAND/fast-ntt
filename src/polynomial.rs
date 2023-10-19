use std::ops::{Add, Mul, Neg, Sub};

pub struct Polynomial {
    pub coef: Vec<i64>,
}

impl Add<Polynomial> for Polynomial {
    type Output = Polynomial;

    fn add(self, rhs: Polynomial) -> Self::Output {
        Polynomial {
            coef: self.coef.iter().zip(rhs.coef).map(|(a, b)| a + b).collect(),
        }
    }
}

impl Sub<Polynomial> for Polynomial {
    type Output = Polynomial;

    fn sub(self, rhs: Polynomial) -> Self::Output {
        self + (-rhs)
    }
}

impl Neg for Polynomial {
    type Output = Polynomial;

    fn neg(self) -> Self::Output {
        Polynomial {
            coef: self.coef.iter().map(|a| -a).collect(),
        }
    }
}

impl Mul<Polynomial> for Polynomial {
    type Output = Polynomial;

    fn mul(self, rhs: Polynomial) -> Self::Output {
        let m = *self.coef.iter().max().unwrap();
        todo!()
    }
}
