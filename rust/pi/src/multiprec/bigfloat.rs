use super::biguint::BigUInt;

#[derive(Debug)]
pub struct BigFloat {
    pub sign: bool,        // (-1) ** sign
    pub exponent: i64,     // BASE ** exponent
    pub fraction: BigUInt, // 整数
}

impl Clone for BigFloat {
    fn clone(&self) -> BigFloat {
        BigFloat {
            sign: self.sign,
            exponent: self.exponent,
            fraction: self.fraction.clone(),
        }
    }
}

impl BigFloat {
    #[inline]
    pub const fn new(sign: bool, exponent: i64, fraction: BigUInt) -> Self {
        BigFloat {
            sign: sign,
            exponent: exponent,
            fraction: fraction,
        }
    }

    pub fn fromBigUInt(n: BigUInt) -> BigFloat {
        BigFloat {
            sign: false,
            exponent: 0,
            fraction: n,
        }
    }

    fn changeExponent(&mut self, nexponent: i64) {
        let diff = self.exponent - nexponent;
        if diff > 0 {
            let mut ret = vec![0; diff as usize];
            ret.append(&mut self.fraction.limbs);
            self.fraction.limbs = ret;
        } else if diff < 0 {
            // 放置
            assert!(false);
        }
    }
}

use std::ops::MulAssign;
impl MulAssign for BigFloat {
    fn mul_assign(&mut self, rhs: BigFloat) {
        self.sign ^= rhs.sign;
        self.exponent += rhs.exponent;
        self.fraction *= rhs.fraction;
    }
}

use std::ops::Mul;
impl Mul for BigFloat {
    type Output = BigFloat;
    fn mul(mut self, rhs: BigFloat) -> Self {
        self.mul_assign(rhs);
        self
    }
}

use std::ops::AddAssign;
impl AddAssign for BigFloat {
    fn add_assign(&mut self, rhs: BigFloat) {
        let min_exponent = if self.exponent < rhs.exponent {
            self.exponent
        } else {
            rhs.exponent
        };

        self.changeExponent(min_exponent);
        rhs.changeExponent(min_exponent);

        if self.sign == rhs.sign {
            self.fraction = self.fraction + rhs.fraction;
        } else if self.fraction < rhs.fraction {
            self.sign = rhs.sign;
            self.fraction = rhs.fraction - self.fraction;
        } else {
            self.fraction = self.fraction - rhs.fraction;
        }
    }
}

use std::ops::Add;
impl Add for BigFloat {
    type Output = Self;
    #[inline]
    fn add(mut self, rhs: BigFloat) -> Self {
        self.add_assign(rhs);
        self
    }
}

use std::ops::SubAssign;
impl SubAssign for BigFloat {
    fn sub_assign(&mut self, rhs: BigFloat) {
        rhs.sign = !rhs.sign;
        *self = *self + rhs;
    }
}

use std::ops::Sub;
impl Sub for BigFloat {
    type Output = BigFloat;
    fn sub(mut self, rhs: BigFloat) -> Self {
        self.sub_assign(rhs);
        self
    }
}

const RECIP_NUM: usize = 5;

pub fn reciprocal(a: BigFloat, prec: i64, init: BigFloat) -> BigFloat {
    let mut ans = init.clone();

    for _i in 0..RECIP_NUM {
        let one = BigFloat {
            sign: false,
            exponent: 0 as i64,
            fraction: BigUInt::new(1),
        };
        ans = ans.clone() + ans.clone() * (one - a.clone() * ans);
    }
    ans
}
