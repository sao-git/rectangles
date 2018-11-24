use num_traits::Pow;
use std::iter::Sum;

//pub fn sum_of_squares<'a, T>(a: &'a [T]) -> T
//where T: Sum<<&'a T as Pow<u32>>::Output>, &'a T: Pow<u32> {
//    a.iter().map(|x: &T| x.pow(2)).sum()
//}

pub fn sum_of_squares<'a, I, T>(vals: I) -> T
where
    I: IntoIterator<Item = &'a T>,
    &'a T: Pow<u32>,
    T: 'a + Sum<<&'a T as Pow<u32>>::Output> {
    vals.into_iter()
        .map(|x| x.pow(2))
        .sum()
}
