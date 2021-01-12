use rug::{ops::Pow, Float, Integer};

// 定数たち
const N: i64 = 10000000;
const SN: i64 = N / 14; // n: small N
const A: i64 = 13591409;
const B: i64 = 545140134;
const C: i64 = 640320;
const CT: i64 = C * C * C;
const CTD24: i64 = CT / 24;

fn calc_x(k: i64) -> Integer {
    if k == 0 {
        return Integer::from(1);
    }
    Integer::from(k).pow(3) * CTD24
}

fn calc_y(k: i64) -> Integer {
    A + Integer::from(B) * k
}

fn calc_z(k: i64) -> Integer {
    if k == SN - 1 {
        return Integer::from(0);
    }
    (-1) * Integer::from((6 * k + 1) * (2 * k + 1)) * (6 * k + 5)
}

fn calc(left: i64, right: i64) -> (Integer, Integer, Integer) {
    if right - left == 1 {
        return (calc_x(left), calc_y(left), calc_z(left));
    }

    let mid = (left + right) >> 1;

    let (lx, ly, lz) = calc(left, mid);
    let (rx, ry, rz) = calc(mid, right);

    (lx * &rx, &rx * ly + ry * &lz, &lz * rz)
}

fn main() {
    // x, y, zの値は間違っていないらしい
    let (x, y, _z) = calc(0, SN);

    // ただansの値がc++版と違う
    let ans = Float::with_val(N as u32 + 20, CT).sqrt() * x / 12 / y;
    println!("{num:.prec$}", prec = N as usize, num = ans);
}
