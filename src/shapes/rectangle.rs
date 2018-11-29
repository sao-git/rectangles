use num_rational::Rational32;
use shapes::ratio::HasRatio;

pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

#[allow(dead_code)]
impl Rectangle {
    pub fn new(width: u32, height: u32) -> Rectangle {
        Rectangle {
            width: width,
            height: height,
        }
    }

    pub fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl HasRatio for Rectangle {
    fn ratio(&self) -> Rational32 {
        // Rational32::new performs reduction, no need for finding gcd
        Rational32::new(self.width as i32, self.height as i32)
    }
}

#[derive(Debug)]
pub struct RealRectangle {
    pub width: f64,
    pub height: f64,
}

/// Will return a positive ratio if it succeeds, -1/1 if not.
impl HasRatio for RealRectangle {
    fn ratio(&self) -> Rational32 {
        match Rational32::approximate_float(self.width / self.height) {
            Some(val) => val,
            None => Rational32::new(-1, 1),
        }
    }
}

impl RealRectangle {
    pub fn new(width: f64, height: f64) -> RealRectangle {
        RealRectangle {
            width: width,
            height: height,
        }
    }

    pub fn area(&self) -> f64 {
        self.width * self.height
    }
}
