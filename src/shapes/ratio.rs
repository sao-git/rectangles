use num_rational::Rational32;

pub trait HasRatio {
    fn ratio(&self) -> Rational32;
}
