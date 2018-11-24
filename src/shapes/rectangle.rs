use num_rational::Rational;
use shapes::ratio::Ratio;

pub struct Rectangle {
    pub height: u32,
    pub width: u32,
}

#[allow(dead_code)]
impl Rectangle {
    pub fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Ratio for Rectangle {
    fn ratio(&self) -> Rational {
        // Rational::new performs reduction, no need for finding gcd
        Rational::new(
            self.width as isize,
            self.height as isize,
        )
    }
}
