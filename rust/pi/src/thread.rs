use rug::{ops::Pow, Float, Integer};
use std::thread;

// 定数たち
const N: i64 = 100000000;
const SN: i64 = N / 14; // n: small N
const A: i64 = 13591409;
const B: i64 = 545140134;
const C: i64 = 640320;
const CT: i64 = C * C * C;
const CTD24: i64 = CT / 24;

// スレッド数
const NTHREADS: usize = 4;

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
    let mut handles = Vec::new();

    let len = (SN + NTHREADS as i64 - 1) / NTHREADS as i64;
    let mut c = SN % len;
    for _x in 1..NTHREADS {
        handles.push(thread::spawn(move || calc(c, c + len)));
        c += len;
    }

    let (fx, fy, fz) = calc(0, SN % len);

    let (x, y, _z) = handles.into_iter().fold((fx, fy, fz), |(x, y, z), handle| {
        let (tx, ty, tz) = handle.join().unwrap();
        (x * &tx, &tx * y + ty * &z, &z * tz)
    });

    // with_valのprocはあくまでも有効桁のビット長(N / log_10^2), 10進数の桁数とは違う
    // 1e8 / log10 ^2の演算結果を四捨五入した値
    let prec: u32 = 332192810;

    // precは10進数1e8桁の場合u32の制限に引っかからない(u32のMAXが4294967295)
    let ans = Float::with_val(prec, CT).sqrt() * x / 12 / y;
    println!("{}", ans);
    // println!("{num:.prec$}", prec = N as usize - 1, num = ans);
}
