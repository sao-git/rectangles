use num_traits::PrimInt;

pub fn sum_of_squares<T>(a: &[T]) -> T
    where T: PrimInt + std::iter::Sum<T> {

    a.iter().map(|x: &T| x.pow(2)).sum()
}
