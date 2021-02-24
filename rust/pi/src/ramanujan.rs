use rug::{ops::Pow, Float, Integer};

// 定数たち
const N: i64 = 100000000;
// const SN: i64 = N / 14; // n: small N
const SN: i64 = 16977929;
const A: i64 = 1123;
const B: i64 = 21460;
const C: i64 = 3528;
const D: i64 = 14112;
const DD: i64 = D * D;

fn calc_x(k: i64) -> Integer {
    if k == 0 {
        return Integer::from(1);
    }
    Integer::from(k).pow(4) * DD
}

fn calc_y(k: i64) -> Integer {
    A + Integer::from(B) * k
}

fn calc_z(k: i64) -> Integer {
    if k == SN - 1 {
        return Integer::from(0);
    }
    (-1) * Integer::from((4 * k + 4) * (4 * k + 3)) * Integer::from((4 * k + 2) * (4 * k + 1))
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

    // with_valのprocはあくまでも有効桁のビット長(N / log_10^2), 10進数の桁数とは違う
    // 1e8 / log10 ^2の演算結果を四捨五入した値
    let prec: u32 = 332192810;

    // precは10進数1e8桁の場合u32の制限に引っかからない(u32のMAXが4294967295)
    let ans = Float::with_val(prec, C) * x / y;
    println!("{}", ans);
    // println!("{num:.prec$}", prec = N as usize - 1, num = ans);
}
