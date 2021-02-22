const BASE_E: usize = 13;
const BASE: i64 = (1 as i64) << BASE_E;
const BASE_MASK: i64 = BASE - 1;

#[derive(Debug)]
pub struct BigUInt {
    pub limbs: Vec<i64>,
}

use std::ops::Add;
impl Add for BigUInt {
    type Output = BigUInt;
    fn add(self, rhs: BigUInt) -> Self::Output {
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
    fn cmp(&self, other: &Self) -> Ordering {
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

fn main() {}
