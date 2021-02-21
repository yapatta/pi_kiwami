const BASE_E: usize = 13;
const BASE: i64 = (1 as i64) << BASE_E;
const BASE_MASK: i64 = BASE - 1;

#[derive(Debug)]
pub struct BigInt {
    pub negative: bool,
    pub limbs: Vec<i64>,
}

use std::ops::Neg;
impl Neg for BigInt {
    type Output = BigInt;
    fn neg(self) -> Self::Output {
        BigInt {
            negative: !self.negative,
            limbs: self.limbs,
        }
    }
}

use std::ops::Add;
impl Add for BigInt {
    type Output = BigInt;
    fn add(self, rhs: BigInt) -> Self::Output {
        if self.negative ^ rhs.negative {
            // ともに符号が同じ場合(大小関係はどうでも良い)
            let limbs_len = if self.limbs.len() >= rhs.limbs.len() {
                self.limbs.len()
            } else {
                rhs.limbs.len()
            };
            let mut limbs = vec![0 as i64; limbs_len];
            for i in 0..self.limbs.len() {
                limbs[i] = self.limbs[i];
            }
            let mut carry: i64 = 0;
            for i in 0..rhs.limbs.len() {
                limbs[i] += rhs.limbs[i] + carry;
                carry = 0;
                if limbs[i] >= BASE {
                    carry = limbs[i] >> BASE_E;
                    limbs[i] &= BASE_MASK;
                }
            }
            while carry > 0 {
                limbs.push(carry & BASE_MASK);
                carry >>= BASE_E;
            }
            return BigInt {
                negative: self.negative,
                limbs: limbs,
            };
        } else {
            // どちらかの符号が違う場合
            // TODO: ここの実装はまずは大小関係が先っぽい(大小関係が重要)
            let limbs_len = if self.limbs.len() > rhs.limbs.len() {
                self.limbs.len()
            } else {
                rhs.limbs.len()
            };
            let mut limbs = vec![0 as i64; limbs_len];
            for i in 0..self.limbs.len() {
                limbs[i] = self.limbs[i];
            }
            let mut carry: i64 = 0;
            for i in 0..rhs.limbs.len() {
                limbs[i] -= rhs.limbs[i] - carry;
                carry = 0;
                if limbs[i] < 0 {
                    carry = 1;
                    limbs[i] += BASE;
                }
            }
            if (carry == 1) {}

            // let negative
            return BigInt {
                negative: self.negative,
                limbs: limbs,
            };
        }
    }
}

use std::ops::Sub;
impl Sub for BigInt {
    type Output = BigInt;
    // TODO: 大小関係を元に実装
    fn sub(self, rhs: BigInt) -> Self::Output {
        let limbs_len = if self.limbs.len() >= rhs.limbs.len() {
            self.limbs.len()
        } else {
            rhs.limbs.len()
        };

        let mut limbs = vec![0 as i64; limbs_len];
        for i in 0..self.limbs.len() {
            limbs[i] = self.limbs[i];
        }

        let mut carry: i64 = 0;
        for i in 0..rhs.limbs.len() {
            limbs[i] -= rhs.limbs[i] - carry;
            carry = 0;
            if limbs[i] < 0 {
                carry = 1;
                limbs[i] += BASE;
            }
        }

        BigInt {
            negative: self.negative,
            limbs: limbs,
        }
    }
}

impl Eq for BigInt {}

impl PartialEq for BigInt {
    fn eq(&self, other: &Self) -> bool {
        let result = self.cmp(other);
        if result == Ordering::Equal {
            return true;
        } else {
            return false;
        }
    }
}

impl PartialOrd for BigInt {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

use std::cmp::Ordering;
impl Ord for BigInt {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.negative == false && self.negative == false {
            if self.limbs.len() > other.limbs.len() {
                return Ordering::Greater;
            } else if self.limbs.len() < other.limbs.len() {
                return Ordering::Less;
            } else {
                for i in (0..self.limbs.len()).rev() {
                    if self.limbs[i] > self.limbs[i] {
                        return Ordering::Greater;
                    } else if self.limbs[i] < self.limbs[i] {
                        return Ordering::Less;
                    }
                }
                return Ordering::Equal;
            }
        } else if self.negative == true && self.negative == true {
            if self.limbs.len() < other.limbs.len() {
                return Ordering::Greater;
            } else if self.limbs.len() > other.limbs.len() {
                return Ordering::Less;
            } else {
                for i in (0..self.limbs.len()).rev() {
                    if self.limbs[i] < self.limbs[i] {
                        return Ordering::Greater;
                    } else if self.limbs[i] > self.limbs[i] {
                        return Ordering::Less;
                    }
                }
                return Ordering::Equal;
            }
        } else if self.negative == false && self.negative == true {
            return Ordering::Greater;
        } else {
            return Ordering::Less;
        }
    }
}

impl BigInt {
    pub fn new(mut a: i64) -> Self {
        let mut al: Vec<i64> = Vec::new();
        if a == 0 {
            al.push(0);
        }
        while a > 0 {
            al.push(a % BASE);
            a /= BASE;
        }
        BigInt {
            negative: false,
            limbs: al,
        }
    }
}

fn main() {}
