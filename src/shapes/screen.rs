use math;
use num_integer::Integer;
use num_rational::Rational32;
use shapes::ratio::HasRatio;
use shapes::rectangle::{Rectangle, RealRectangle};

pub struct Screen {
    diagonal: f64,
    pixel_aspect: Rational32,
    dimensions: Rectangle,
}

impl HasRatio for Screen {
    fn ratio(&self) -> Rational32 {
        self.dimensions.ratio()
    }
}

impl Screen {
    pub fn new_square(diagonal: f64, width: u32, height: u32) -> Screen {
        Screen {
            diagonal: diagonal,
            pixel_aspect: Rational32::new(1, 1),
            dimensions: Rectangle {
                width: width,
                height: height,
            },
        }
    }

    pub fn new(diagonal: f64, width: u32, height: u32,
               pixel_width: u32, pixel_height: u32) -> Screen {
        Screen {
            diagonal: diagonal,
            pixel_aspect: Rational32::new(
                pixel_width as i32,
                pixel_height as i32
            ),
            dimensions: Rectangle {
                width: width,
                height: height,
            },
        }
    }

    /// x:y is (when reduced) the picture aspect ratio, equal to:
    ///
    /// _number of horizontal pixels * pixel aspect width_
    /// number of vertical pixels * pixel aspect height
    ///
    /// a is the proportionality constant that relates the diagonal to
    /// the sides of the screen in units of length
    pub fn side_lengths(&self) -> RealRectangle {
        // Get initial x and y
        let x = self.dimensions.width * *self.pixel_aspect.numer() as u32;
        let y = self.dimensions.height * *self.pixel_aspect.denom() as u32;

        // Reduce to a ratio
        let gcd = x.gcd(&y);
        let x = x / gcd;
        let y = y / gcd;

        // Calculate `a`
        let sum = math::sum_pow(&[x, y], 2) as f64;
        let a = self.diagonal / sum.sqrt();

        RealRectangle::new(x as f64 * a, y as f64 * a)
    }

    pub fn area(&self) -> f64 {
        let sides = self.side_lengths();
        sides.width * sides.height
    }
}
