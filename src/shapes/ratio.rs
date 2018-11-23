use num_rational::Rational;

pub trait Ratio {
    fn ratio(&self) -> Rational;
}

