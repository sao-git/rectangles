use rectangle;

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

fn sum_of_squares<T>(a: &[T]) -> T
    where T: PrimInt + std::iter::Sum<T> {

    a.iter().map(|x: &T| x.pow(2)).sum()
}

