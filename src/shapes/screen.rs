use math;
use num_rational::Rational32;
use shapes::ratio::HasRatio;
use shapes::rectangle::{Rectangle, RealRectangle};

pub struct Screen {
    diagonal: f64,
    dimensions: Rectangle,
}

impl HasRatio for Screen {
    fn ratio(&self) -> Rational32 {
        self.dimensions.ratio()
    }
}

impl Screen {
    pub fn new(diagonal: f64, width: u32, height: u32) -> Screen {
        Screen {
            diagonal: diagonal,
            dimensions: Rectangle {
                width: width,
                height: height,
            },
        }
    }

    // Helper function
    //
    // x:y is the aspect ratio (assuming square pixels)
    //
    // a is the proportionality constant that relates the diagonal to
    // the sides in units of length
    fn alpha(&self) -> (u32, u32, f64) {
        let ratio = self.dimensions.ratio();
        let x = *ratio.numer() as u32;
        let y = *ratio.denom() as u32;
        let sum = math::sum_pow(&[x, y], 2) as f64;
        let a = self.diagonal / sum.sqrt();

        (x, y, a)
    }

    pub fn side_lengths(&self) -> RealRectangle {
        let (x, y, a) = self.alpha();
        RealRectangle {
            width: x as f64 * a,
            height: y as f64 * a,
        }
    }

    pub fn area(&self) -> f64 {
        let (x, y, a) = self.alpha();
        let product = (x * y) as f64;

        product * a.powi(2)
    }
}
