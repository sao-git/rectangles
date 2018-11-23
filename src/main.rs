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

struct Screen {
    diagonal: f64,
    dimensions: Rectangle,
}

impl Ratio for Screen {
    fn ratio(&self) -> Rational {
        self.dimensions.ratio()
    }
}

impl Screen {
    fn new(diagonal: f64, width: u32, height: u32) -> Screen {
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

    fn area(&self) -> f64 {
        let ratio = self.dimensions.ratio();
        let width_ratio = *ratio.numer();
        let height_ratio = *ratio.denom();
        let product = (width_ratio * height_ratio) as f64;
        let sum_2 = sum_of_squares(&[
            height_ratio,
            width_ratio
        ]) as f64;
        let x = self.diagonal / sum_2.sqrt();

        product * x.powf(2.0)
    }
}

fn main() {
    macro_rules! screen_print {
        () => ("The area of the screen is {:.4} square inches.")
    };

    let screen1 = Screen::new(
        7.0, 800, 480
    );
    println!(screen_print!(), screen1.area());

    let screen2 = Screen::new(
        3.2, 320, 240
    );
    println!(screen_print!(), screen2.area());
}

fn sum_of_squares<T>(a: &[T]) -> T
    where T: PrimInt + std::iter::Sum<T> {

    a.iter().map(|x: &T| x.pow(2)).sum()
}

