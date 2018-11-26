use num_traits::Pow;
use std::iter::Sum;

/// Returns the sum of the squared values in an iterable.
///
/// # Arguments
///
/// * `I` - Must have the `IntoIterator` trait
/// * `T` - A numeric type that has the `num_traits::Pow<u8>` trait
///
/// # Example
///
/// ```
/// let numbers = vec![2.3, 4.5, -23333.0123];
/// let numbers_2 = math::sum_of_squares(&numbers);
/// let numbers_3 = math::sum_of_squares(&[1.414, 1.414]);
/// ```
pub fn sum_of_squares<'a, I, T>(vals: I) -> T
where
    I: IntoIterator<Item = &'a T>,
    &'a T: Pow<u8>,
    T: 'a + Sum<<&'a T as Pow<u8>>::Output> {
    vals.into_iter()
        .map(|x| x.pow(2))
        .sum()
}
