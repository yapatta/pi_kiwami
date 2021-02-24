use num::complex::Complex;
use std::f64::consts::PI;

pub fn fftr(f: &mut Vec<Complex<f64>>, k: usize) {
    if k == 0 {
        return;
    }

    let n = 1 << k;
    let mut fe = vec![Complex::new(0 as f64, 0 as f64); n / 2];
    let mut fo = vec![Complex::new(0 as f64, 0 as f64); n / 2];

    for i in 0..n / 2 {
        fe[i] = f[2 * i];
        fo[i] = f[2 * i + 1];
    }

    fftr(&mut fe, k - 1);
    fftr(&mut fo, k - 1);

    for i in 0..n / 2 {
        f[i] = fe[i]
            + fo[i]
                * Complex::exp(
                    Complex::new(-(i as f64 * PI * 2 as f64) / n as f64, 0 as f64) * Complex::i(),
                );
    }

    for i in 0..n / 2 {
        f[i + n / 2] = fe[i]
            - fo[i]
                * Complex::exp(
                    Complex::new(-(i as f64 * PI * 2 as f64) / n as f64, 0 as f64) * Complex::i(),
                );
    }
}

// aをfftr, bをfftrして
pub fn convolve(a: Vec<i64>, b: Vec<i64>, n: usize) -> Vec<i64> {
    let mut l = 0;
    let mut k = 1;
    while k < n {
        k <<= 1;
        l += 1;
    }
    k <<= 1;
    l += 1;

    let mut af = vec![Complex::new(0 as f64, 0 as f64); k];
    for i in 0..n {
        af[i] = Complex::new(a[i] as f64, 0 as f64);
    }
    fftr(&mut af, l);

    let mut bf = vec![Complex::new(0 as f64, 0 as f64); k];
    for i in 0..n {
        bf[i] = Complex::new(b[i] as f64, 0 as f64);
    }
    fftr(&mut bf, l);

    let mut cf = vec![Complex::new(0 as f64, 0 as f64); k];
    for i in 0..k {
        cf[i] = af[i] * bf[i];
        cf[i] = Complex::conj(&cf[i]);
    }

    fftr(&mut cf, l);

    let mut retcf: Vec<i64> = Vec::new();
    for i in 0..cf.len() {
        retcf.push(cf[i].re.round() as i64 / k as i64);
    }

    retcf
}
