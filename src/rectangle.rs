extern crate num;

use num::{PrimInt, Integer, Rational};

pub trait Ratio {
    fn ratio(&self) -> Rational;
}

struct Rectangle {
    height: u32,
    width: u32,
}

//impl Rectangle {
//}

impl Ratio for Rectangle {
    fn ratio(&self) -> Rational
    where u32: Integer {
        let gcd = self.width.gcd(&self.height);
        Rational::new(
            (self.width / gcd) as isize,
            (self.height / gcd) as isize
        )
    }
}
