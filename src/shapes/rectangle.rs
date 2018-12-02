use num_rational::Ratio;
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

    pub fn area(&self) -> usize {
        self.width as usize * self.height as usize
    }
}

impl HasRatio for Rectangle {
    fn ratio(&self) -> Ratio<u32> {
        // Rational32::new performs reduction, no need for finding gcd
        Ratio::new(self.width, self.height)
    }
}

#[derive(Debug)]
pub struct RealRectangle {
    pub width: f64,
    pub height: f64,
}

impl HasRatio for RealRectangle {
    fn ratio(&self) -> Ratio<u32> {
        let t: Ratio<i32> = Ratio::approximate_float(self.width / self.height)
            .unwrap();
        Ratio::new_raw(*t.numer() as u32, *t.denom() as u32)
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
