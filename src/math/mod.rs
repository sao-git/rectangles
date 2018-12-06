use num_traits::Pow;
use std::iter::Sum;
use rayon::prelude::*;

pub fn percent_diff<T>(x: T, y: T) -> f64
where f64: From<T> {
    (f64::from(x) / f64::from(y) - 1.0) * 100.0
}

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
pub fn sum_pow<I>(vals: I, power: u8) -> f64
where
    I: IntoParallelIterator + Copy,
    <I as IntoParallelIterator>::Item: Pow<u8>,
    <<I as IntoParallelIterator>::Item as Pow<u8>>::Output: Send,
    f64: Sum<<I as IntoParallelIterator>::Item> +
    Sum<<<I as IntoParallelIterator>::Item as Pow<u8>>::Output> {

        let iter = vals.into_par_iter();
        match power {
            0 => iter.count() as f64, // Treating 0 to the 0th as 1
            1 => iter.sum(),
            _ => iter.map(|x| x.pow(power)).sum(),
        }
}

/// The “generalized mean”, i.e. where each element of `I` is raised to a
/// `power`, then summed, then divided by the number of elements.
///
/// Will use `power = 1` if default arguments are ever implemented for a
/// “normal” average.
pub fn mean<I>(vals: I, power: u8) -> f64
where
    I: IntoParallelIterator + Copy,
    <I as IntoParallelIterator>::Item: Pow<u8>,
    <<I as IntoParallelIterator>::Item as Pow<u8>>::Output: Send,
    f64: Sum<<I as IntoParallelIterator>::Item> +
    Sum<<<I as IntoParallelIterator>::Item as Pow<u8>>::Output> {

        let count = vals.into_par_iter().count() as f64;
        let sum = sum_pow(vals, power);
        sum / count
}

/// Root mean square, the square root of the square mean of the elements of I.
pub fn rms<I>(vals: I) -> f64
where
    I: IntoParallelIterator + Copy,
    <I as IntoParallelIterator>::Item: Pow<u8>,
    <<I as IntoParallelIterator>::Item as Pow<u8>>::Output: Send,
    f64: Sum<<I as IntoParallelIterator>::Item> +
    Sum<<<I as IntoParallelIterator>::Item as Pow<u8>>::Output> {

        mean(vals, 2).sqrt()
}
