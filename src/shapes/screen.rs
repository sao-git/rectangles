use num_integer::Integer;
use num_rational::Ratio;
use shapes::ratio::HasRatio;
use shapes::rectangle::{Rectangle, RealRectangle};
use std::str::FromStr;

pub struct Screen {
    diagonal: String,
    pixel_aspect: Ratio<u32>,
    pixel_dimensions: Rectangle,
}

impl HasRatio for Screen {
    fn ratio(&self) -> Ratio<u32> {
        self.pixel_dimensions.ratio()
    }
}

impl Screen {
    pub fn new_square(diagonal: f64, width: u32, height: u32) -> Screen {
        Screen {
            diagonal: fomat!({diagonal:e}),
            pixel_aspect: Ratio::new(1, 1),
            pixel_dimensions: Rectangle {
                width: width,
                height: height,
            },
        }
    }

    pub fn new(diagonal: f64, width: u32, height: u32,
               pixel_width: u32, pixel_height: u32) -> Screen {
        Screen {
            diagonal: fomat!({diagonal:e}),
            pixel_aspect: Ratio::new(pixel_width, pixel_height),
            pixel_dimensions: Rectangle {
                width: width,
                height: height,
            },
        }
    }

    pub fn diagonal(&self) -> f64 {
        f64::from_str(&self.diagonal).unwrap()
    }

    pub fn pixel_aspect(&self) -> &Ratio<u32> {
        &self.pixel_aspect
    }

    pub fn pixel_dimensions(&self) -> &Rectangle {
        &self.pixel_dimensions
    }

    pub fn side_lengths(&self) -> RealRectangle {
        let (x, y, a) = alpha(
            self.pixel_dimensions.width,
            self.pixel_dimensions.height,
            self.pixel_aspect,
            self.diagonal.clone()
        );
        let a = f64::from_str(&a).unwrap();
        RealRectangle::new(x as f64 * a, y as f64 * a)
    }

    pub fn area(&self) -> f64 {
        let sides = self.side_lengths();
        sides.width * sides.height
    }
}

// Memoized helper function `alpha()`
//
// `x`:`y` is (when reduced) the picture aspect ratio, equal to:
//
// _number of horizontal pixels * pixel aspect width_
// number of vertical pixels * pixel aspect height
//
// `a` is the proportionality constant that relates the diagonal to
// the sides of the screen in units of length
//
// The input and output `f64`s are wrapped in `String` instances to satisfy
// the `Eq` and `HashMap` trait requirements on `cached!` functions.
cached!{
    SCREENALPHA;
    fn alpha(w: u32, h: u32, p: Ratio<u32>, diag: String)
        -> (u32, u32, String) = {
        // Get initial x and y
        let (w_p, h_p) = (*p.numer(), *p.denom());
        let (x, y) = (w * w_p, h * h_p);

        // Reduce to a ratio
        let gcd = x.gcd(&y);
        let (x, y) = (x / gcd, y / gcd);

        // Calculate a and return (x, y, a)
        let sum = (x.pow(2) + y.pow(2)) as f64;
        let a = f64::from_str(&diag).unwrap() / sum.sqrt();
        (x, y, fomat!({a:e}))
    }
}
