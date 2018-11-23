use num_integer::Integer;
use num_rational::Rational;
use shapes::ratio::Ratio;

pub struct Rectangle {
    pub height: u32,
    pub width: u32,
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
