use std::cmp;
use std::fmt;
use std::ops;

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct Frac {
    num: u32,
    den: u32,
    neg: bool,
}

impl Frac {
    pub fn new(num: i32, den: i32) -> Self {
        let gcd = Self::gcd(num.abs() as u32, den.abs() as u32);
        Self {
            num: num.abs() as u32 / gcd,
            den: den.abs() as u32 / gcd,
            neg: (num < 0) ^ (den < 0),
        }
    }

    pub fn from_u(num: u32, den: u32, neg: bool) -> Self {
        let gcd = Self::gcd(num, den);
        Self {
            num: num / gcd,
            den: den / gcd,
            neg,
        }
    }

    pub fn from_i(num: i32) -> Self {
        Self {
            num: num.abs() as u32,
            den: 1,
            neg: (num < 0),
        }
    }

    pub fn is_neg(&self) -> bool {
        self.neg
    }

    fn gcd(mut num: u32, mut den: u32) -> u32 {
        if num == 0 && den == 0 {
            1
        } else if num == 0 {
            den
        } else if den == 0 {
            num
        } else {
            let mut prod: u32 = 1;
            let mut i = 2;
            while i <= cmp::min(num, den) {
                let a: u32 = num % i;
                let b: u32 = den % i;
                if a == 0 {
                    num /= i;
                }
                if b == 0 {
                    den /= i;
                }
                if a == 0 && b == 0 {
                    prod *= i;
                } else if a != 0 && b != 0 {
                    i += 1;
                }
            }
            prod
        }
    }
}

impl ops::Add<Frac> for Frac {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let mut a: i32 = 1;
        let mut b: i32 = 1;
        if self.neg {
            a = -1
        }
        if rhs.neg {
            b = -1
        }
        Frac::new(
            a * self.num as i32 * rhs.den as i32 + b * rhs.num as i32 * self.den as i32,
            self.den as i32 * rhs.den as i32,
        )
    }
}

impl ops::Sub<Frac> for Frac {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        let mut a: i32 = 1;
        let mut b: i32 = 1;
        if self.neg {
            a = -1
        }
        if rhs.neg {
            b = -1
        }
        Frac::new(
            a * self.num as i32 * rhs.den as i32 - b * rhs.num as i32 * self.den as i32,
            self.den as i32 * rhs.den as i32,
        )
    }
}

impl ops::Mul<Frac> for Frac {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Frac::from_u(self.num * rhs.num, self.den * rhs.den, self.neg ^ rhs.neg)
    }
}

impl ops::Div<Frac> for Frac {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Frac::from_u(self.num * rhs.den, self.den * rhs.num, self.neg ^ rhs.neg)
    }
}

impl ops::Neg for Frac {
    type Output = Self;

    fn neg(self) -> Self {
        Frac::from_u(self.num, self.den, !self.neg)
    }
}

impl cmp::PartialOrd for Frac {
    fn partial_cmp(&self, other: &Frac) -> Option<cmp::Ordering> {
        let frac: Frac = *self - *other;
        if frac == Frac::new(0, 1) {
            Some(cmp::Ordering::Equal)
        } else if frac.is_neg() {
            Some(cmp::Ordering::Less)
        } else {
            Some(cmp::Ordering::Greater)
        }
    }
}

impl fmt::Display for Frac {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut neg: char = '+';
        if self.is_neg() {
            neg = '-';
        }
        write!(f, "{}{}/{}", neg, self.num, self.den)
    }
}

#[cfg(test)]
mod test {
    use super::Frac;

    #[test]
    fn gcd() {
        let frac = Frac::new(5, 10);
        assert_eq!(frac, Frac::new(1, 2));

        let frac = Frac::new(2, 2);
        assert_eq!(frac, Frac::new(1, 1));
    }

    #[test]
    fn add() {
        let a = Frac::new(5, 7);
        let b = Frac::new(2, 7);
        assert_eq!(a + b, Frac::new(1, 1));

        let a = Frac::new(-5, 7);
        let b = Frac::new(-2, 7);
        assert_eq!(a + b, Frac::new(-1, 1));

        let a = Frac::new(-2, 7);
        let b = Frac::new(2, 7);
        assert_eq!(a + b, Frac::new(0, 1));
    }

    #[test]
    fn sub() {
        let a = Frac::new(5, 7);
        let b = Frac::new(5, 7);
        assert_eq!(a - b, Frac::new(0, 1));

        let a = Frac::new(5, 7);
        let b = Frac::new(-2, 7);
        assert_eq!(a - b, Frac::new(1, 1));

        let a = Frac::new(-5, 7);
        let b = Frac::new(2, 7);
        assert_eq!(a - b, Frac::new(-1, 1));

        let a = Frac::new(-5, 7);
        let b = Frac::new(-5, 7);
        assert_eq!(a - b, Frac::new(0, 1));
    }

    #[test]
    fn mul() {
        let a = Frac::new(6, 7);
        let b = Frac::new(7, 3);
        assert_eq!(a * b, Frac::new(2, 1));

        let a = Frac::new(-6, 7);
        let b = Frac::new(7, 3);
        assert_eq!(a * b, Frac::new(-2, 1));

        let a = Frac::new(6, 7);
        let b = Frac::new(7, -3);
        assert_eq!(a * b, Frac::new(-2, 1));

        let a = Frac::new(6, -7);
        let b = Frac::new(-7, 3);
        assert_eq!(a * b, Frac::new(2, 1));
    }

    #[test]
    fn div() {
        let a = Frac::new(6, 7);
        let b = Frac::new(3, 7);
        assert_eq!(a / b, Frac::new(2, 1));

        let a = Frac::new(-6, 7);
        let b = Frac::new(3, 7);
        assert_eq!(a / b, Frac::new(-2, 1));

        let a = Frac::new(6, 7);
        let b = Frac::new(3, -7);
        assert_eq!(a / b, Frac::new(-2, 1));

        let a = Frac::new(6, -7);
        let b = Frac::new(-3, 7);
        assert_eq!(a / b, Frac::new(2, 1));
    }

    #[test]
    fn zero() {
        let frac = Frac::new(0, 6);
        assert_eq!(frac, Frac::new(0, 4));
        assert_eq!(frac.num, 0);
        assert_eq!(frac.den, 1);

        let frac = Frac::new(6, 0);
        assert_eq!(frac, Frac::new(4, 0));
        assert_eq!(frac.num, 1);
        assert_eq!(frac.den, 0);

        let frac = Frac::new(0, 0);
        assert_eq!(frac.num, 0);
        assert_eq!(frac.den, 0);
    }

    #[test]
    fn neg() {
        let frac = Frac::new(-1, 1);
        assert_eq!(frac, Frac::new(1, -1));

        let frac = Frac::new(-1, -1);
        assert_eq!(frac, Frac::new(1, 1));
    }
}
