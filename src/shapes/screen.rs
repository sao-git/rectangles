use math;
use num_rational::Rational;
use shapes::ratio::Ratio;
use shapes::rectangle::Rectangle;

pub struct Screen {
    diagonal: f64,
    dimensions: Rectangle,
}

impl Ratio for Screen {
    fn ratio(&self) -> Rational {
        self.dimensions.ratio()
    }
}

impl Screen {
    pub fn new(diagonal: f64, width: u32, height: u32)
        -> Screen {
        Screen {
            diagonal: diagonal,
            dimensions: Rectangle {
                width: width,
                height: height,
            }
        }
    }

    //fn side_length(&self) -> Rectangle {
    //}

    pub fn area(&self) -> f64 {
        let ratio = self.dimensions.ratio();
        let width_ratio = *ratio.numer() as u32;
        let height_ratio = *ratio.denom() as u32;
        let product = (width_ratio * height_ratio) as f64;
        let sum = math::sum_pow(&[
            height_ratio,
            width_ratio
        ], 2) as f64;
        let x = self.diagonal / sum.sqrt();

        product * x.powf(2.0)
    }
}
