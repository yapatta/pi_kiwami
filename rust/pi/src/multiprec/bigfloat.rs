use super::biguint::BigUInt;

#[derive(Debug)]
pub struct BigFloat {
    pub sign: bool, // (-1) ** sign
    pub exponent: i64,
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

use std::ops::Mul;
impl Mul for BigFloat {
    type Output = BigFloat;
    fn mul(self, rhs: Self) -> Self::Output {
        BigFloat {
            sign: self.sign ^ rhs.sign,
            exponent: self.exponent + rhs.exponent,
            fraction: self.fraction * rhs.fraction,
        }
    }
}

impl BigFloat {
    pub fn new(sign: bool, exponent: i64, fraction: BigUInt) -> Self {
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

    fn changeExponent(&self, nexponent: i64) -> Self {
        let diff = self.exponent - nexponent;
        let mut ret = vec![0; diff as usize];
        if diff > 0 {
            for l in self.fraction.limbs.iter() {
                ret.push(*l)
            }
        } else if diff < 0 {
            // 放置
            assert!(false);
        } else {
            return self.clone();
        }
        BigFloat::new(self.sign, nexponent, BigUInt { limbs: ret })
    }
}

use std::ops::Add;
impl Add for BigFloat {
    type Output = BigFloat;
    fn add(self, rhs: Self) -> Self::Output {
        let min_exponent = if self.exponent < rhs.exponent {
            self.exponent
        } else {
            rhs.exponent
        };

        let ret_self = self.changeExponent(min_exponent);
        let ret_rhs = rhs.changeExponent(min_exponent);

        if ret_self.sign == ret_rhs.sign {
            return BigFloat {
                sign: ret_self.sign,
                exponent: min_exponent,
                fraction: ret_self.fraction + ret_rhs.fraction,
            };
        } else if ret_self.fraction < ret_rhs.fraction {
            return BigFloat {
                sign: ret_rhs.sign,
                exponent: min_exponent,
                fraction: ret_rhs.fraction - ret_self.fraction,
            };
        } else {
            return BigFloat {
                sign: ret_self.sign,
                exponent: min_exponent,
                fraction: ret_self.fraction - ret_rhs.fraction,
            };
        }
    }
}

use std::ops::Sub;
impl Sub for BigFloat {
    type Output = BigFloat;
    fn sub(self, rhs: Self) -> Self::Output {
        let neg_rhs = BigFloat::new(!rhs.sign, rhs.exponent, rhs.fraction);
        self + neg_rhs
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
