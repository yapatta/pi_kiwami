use libm::log10;
use std::env;

use pi::multiprec::bigfloat::{reciprocal, BigFloat};
use pi::multiprec::biguint::{get_number, BigUInt, BASE};

const A: i64 = 13591409;
const B: i64 = 545140134;
const C: i64 = 640320;
const CT: i64 = C * C * C;
const CT24: i64 = C * C * C / 24;

#[derive(Debug)]
struct M {
    X: BigUInt,
    Y: BigUInt,
    Z: BigUInt,
}

fn calc_x(k: i64) -> BigUInt {
    if k == 0 {
        return BigUInt::new(1);
    }
    BigUInt::new(k) * BigUInt::new(k) * BigUInt::new(k) * BigUInt::new(CT24)
}

fn calc_y(k: i64) -> BigUInt {
    BigUInt::new(A) + BigUInt::new(B) * BigUInt::new(k)
}

fn calc_z(k: i64, n: i64) -> BigUInt {
    if k == n - 1 {
        return BigUInt::new(0);
    }
    BigUInt::new(6 * k + 1) * BigUInt::new(2 * k + 1) * BigUInt::new(6 * k + 5)
}

fn mul(lm: M, rm: M) -> M {
    M {
        X: lm.X * rm.X.clone(),
        Y: lm.Y * rm.X + lm.Z.clone() * rm.Y,
        Z: lm.Z * rm.Z,
    }
}

fn calc(n: i64, left: i64, right: i64) -> M {
    if right - left == 1 {
        return M {
            X: calc_x(left),
            Y: calc_y(left),
            Z: calc_z(left, n),
        };
    }

    let mid = (left + right) >> 1;
    let lm = calc(n, left, mid);
    let rm = calc(n, mid, right);

    mul(lm, rm)
}

fn main() {
    let AA = BigUInt::new(A);
    let BB = BigUInt::new(B);
    let args: Vec<String> = env::args().collect();

    let N: i64 = if args.len() < 2 {
        println!("no argument is set");
        1000
    } else {
        args[1].parse().unwrap()
    };

    let n = (N / 14) + (N / 14) % 2;

    let m = calc(n, 0, n);
    println!("precision: {:?}", N);
    println!("m: {:?}", m);

    let prec = (N as f64 / log10(BASE as f64)).ceil() as i64;

    let ans = BigFloat::fromBigUInt(BigUInt::new(4270934400) * m.X)
        * of
        * reciprocal(
            BigFloat::fromBigUInt(m.Y),
            prec,
            BigFloat::new(false, -1, BigUInt::new(2048)),
        );

    let a = BigFloat::new(false, 0, BigUInt::new(2));
    let b = BigFloat::new(false, 0, BigUInt::new(7));
    let c = a * reciprocal(b, 0, BigFloat::new(false, -1, BigUInt::new(2048)));
    println!("{:?}", ans);
}
