use num_rational::Ratio;

pub trait HasRatio {
    fn ratio(&self) -> Ratio<u32>;
}
