use pi::multiprec::bigfloat::{reciprocal, BigFloat};
use pi::multiprec::biguint::{get_number, BigUInt};

fn main() {
    let a = BigFloat::new(false, 0, BigUInt::new(2));
    let b = BigFloat::new(false, 0, BigUInt::new(7));
    let c = a * reciprocal(b, 0, BigFloat::new(false, -1, BigUInt::new(2048)));
    println!("{:?}", c);
}
