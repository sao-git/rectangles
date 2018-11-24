use num_traits::Pow;
use std::iter::Sum;

pub fn sum_of_squares<'a, T>(a: &'a [T]) -> T
where T: Sum<<&'a T as Pow<u32>>::Output>, &'a T: Pow<u32> {
    a.iter().map(|x: &T| x.pow(2)).sum()
}
