use super::fft::{convolve, convolve_mut};

const BASE_E: usize = 13;
const BASE: i64 = (1 as i64) << BASE_E;
const BASE_MASK: i64 = BASE - 1;

#[derive(Debug)]
pub struct BigUInt {
    pub limbs: Vec<i64>,
}

impl Clone for BigUInt {
    fn clone(&self) -> BigUInt {
        let mut limbs = vec![0; self.limbs.len()];
        for i in 0..self.limbs.len() {
            limbs[i] = self.limbs[i];
        }
        BigUInt { limbs: limbs }
    }
}

use std::ops::Add;
impl Add for BigUInt {
    type Output = BigUInt;
    fn add(self, rhs: BigUInt) -> Self::Output {
        let max_len = if self.limbs.len() >= rhs.limbs.len() {
            self.limbs.len()
        } else {
            rhs.limbs.len()
        };

        let mut limbs: Vec<i64> = vec![0; max_len];
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
        return BigUInt { limbs: limbs };
    }
}

use std::ops::AddAssign;
impl AddAssign for BigUInt {
    fn add_assign(&mut self, rhs: Self) {
        if self.limbs.len() < rhs.limbs.len() {
            let mut v = vec![0 as i64; rhs.limbs.len() - self.limbs.len()];
            self.limbs.append(&mut v);
        };
        let mut carry: i64 = 0;
        for i in 0..rhs.limbs.len() {
            self.limbs[i] += rhs.limbs[i] + carry;
            carry = 0;
            if self.limbs[i] >= BASE {
                carry = self.limbs[i] >> BASE_E;
                self.limbs[i] &= BASE_MASK;
            }
        }
        while carry > 0 {
            self.limbs.push(carry & BASE_MASK);
            carry >>= BASE_E;
        }
    }
}

use std::ops::Sub;
impl Sub for BigUInt {
    type Output = BigUInt;
    // if self < rhs, result is undefined
    fn sub(self, rhs: BigUInt) -> Self::Output {
        let max_len = if self.limbs.len() >= rhs.limbs.len() {
            self.limbs.len()
        } else {
            rhs.limbs.len()
        };

        let mut limbs: Vec<i64> = vec![0; max_len];
        for i in 0..self.limbs.len() {
            limbs[i] = self.limbs[i];
        }

        let mut carry: i64 = 0;
        for i in 0..rhs.limbs.len() {
            limbs[i] -= rhs.limbs[i] + carry;
            carry = 0;
            if limbs[i] < 0 {
                carry = 1;
                limbs[i] += BASE;
            }
        }

        if carry == 1 {
            if limbs.len() > rhs.limbs.len() {
                limbs[rhs.limbs.len()] -= carry;
                carry = 0;
            } else {
                panic!("undefined");
            }
        }

        assert_eq!(carry, 0);

        BigUInt { limbs: limbs }
    }
}

use std::ops::SubAssign;
impl SubAssign for BigUInt {
    fn sub_assign(&mut self, rhs: Self) {
        assert_eq!(*self >= rhs, true);

        let mut carry: i64 = 0;
        for i in 0..rhs.limbs.len() {
            self.limbs[i] -= rhs.limbs[i] - carry;
            carry = 0;
            if self.limbs[i] < 0 {
                carry = 1;
                self.limbs[i] += BASE;
            }
        }
        assert_eq!(carry, 0);
    }
}

use std::ops::Mul;
impl Mul for BigUInt {
    type Output = BigUInt;
    // if self < rhs, result is undefined
    fn mul(self, rhs: BigUInt) -> Self::Output {
        let max_len = if self.limbs.len() > rhs.limbs.len() {
            self.limbs.len()
        } else {
            rhs.limbs.len()
        };

        let c = convolve(self.limbs, rhs.limbs, max_len);

        BigUInt {
            limbs: normalize(c),
        }
    }
}

use std::ops::MulAssign;
impl MulAssign for BigUInt {
    fn mul_assign(&mut self, rhs: Self) {
        assert_eq!(self.limbs.len(), rhs.limbs.len());
        let max_len = if self.limbs.len() > rhs.limbs.len() {
            self.limbs.len()
        } else {
            rhs.limbs.len()
        };
        self.limbs.resize_with(max_len, Default::default);

        convolve_mut(&mut self.limbs, rhs.limbs, max_len);
        normalize_mut(&mut self.limbs);
    }
}

impl Eq for BigUInt {}

impl PartialEq for BigUInt {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl PartialOrd for BigUInt {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

use std::cmp::Ordering;
impl Ord for BigUInt {
    fn cmp(&self, rhs: &Self) -> Ordering {
        let mut flag = false;
        let max_len = if self.limbs.len() >= rhs.limbs.len() {
            self.limbs.len()
        } else {
            rhs.limbs.len()
        };

        let mut limbs: Vec<i64> = vec![0; max_len];
        for i in 0..self.limbs.len() {
            limbs[i] = self.limbs[i];
        }

        let mut carry: i64 = 0;
        for i in 0..rhs.limbs.len() {
            limbs[i] -= rhs.limbs[i] + carry;
            carry = 0;
            if limbs[i] < 0 {
                carry = 1;
                limbs[i] += BASE;
            }
            if limbs[i] != 0 {
                flag = true;
            }
        }

        if carry == 0 && !flag {
            return Ordering::Equal;
        }
        if carry == 1 {
            if limbs.len() > rhs.limbs.len() {
                limbs[rhs.limbs.len()] -= carry;
                carry = 0;
            } else {
                return Ordering::Less;
            }
        }

        Ordering::Greater
    }
}

impl BigUInt {
    pub fn new(mut a: i64) -> Self {
        let mut al: Vec<i64> = Vec::new();
        if a == 0 {
            al.push(0);
        }
        while a > 0 {
            al.push(a % BASE);
            a /= BASE;
        }
        BigUInt { limbs: al }
    }

    pub fn len(self) -> usize {
        self.limbs.len()
    }
}

fn normalize(limbs: Vec<i64>) -> Vec<i64> {
    let mut carry = 0;
    let mut msl = 0;
    let mut normalized_limbs = vec![0 as i64; limbs.len()];

    for i in 0..limbs.len() {
        normalized_limbs[i] = limbs[i] + carry;
        carry = 0;
        if normalized_limbs[i] >= BASE {
            carry = normalized_limbs[i] >> BASE_E;
            normalized_limbs[i] &= BASE_MASK;
        }
        if normalized_limbs[i] != 0 {
            msl = i;
        }
    }
    while carry > 0 {
        let cur = carry & BASE_MASK;
        normalized_limbs.push(cur);
        carry >>= BASE_E;
        if cur != 0 {
            msl = normalized_limbs.len() - 1;
        }
    }

    if normalized_limbs.len() > msl * 3 {
        normalized_limbs.resize_with(msl + 2, Default::default);
    }

    normalized_limbs
}

fn normalize_mut(limbs: &mut Vec<i64>) {
    let mut carry = 0;
    let mut msl = 0;

    for i in 0..limbs.len() {
        limbs[i] = limbs[i] + carry;
        carry = 0;
        if limbs[i] >= BASE {
            carry = limbs[i] >> BASE_E;
            limbs[i] &= BASE_MASK;
        }
        if limbs[i] != 0 {
            msl = i;
        }
    }
    while carry > 0 {
        let cur = carry & BASE_MASK;
        limbs.push(cur);
        carry >>= BASE_E;
        if cur != 0 {
            msl = limbs.len() - 1;
        }
    }

    if limbs.len() > msl * 3 {
        limbs.resize_with(msl + 2, Default::default);
    }
}

pub fn get_number(c: &BigUInt) -> i64 {
    let mut sum = 0;
    for i in (0..c.limbs.len()).rev() {
        sum = sum * BASE + c.limbs[i];
    }
    sum
}

// TODO: あとでやる
pub fn get_str(c: &BigUInt) -> String {
    let mut ret = "".to_string();
    let mut tmp_sum = 0;
    for i in (0..c.limbs.len()).rev() {
        tmp_sum = tmp_sum * BASE + c.limbs[i];
        let tmp = (tmp_sum / 1000).to_string();
        tmp_sum %= 1000;
        ret = tmp + &ret;
    }
    ret
}
