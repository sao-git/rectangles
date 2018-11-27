use num_traits::Pow;
use std::iter::Sum;
use std::ops::Div;


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
pub fn sum_pow<'a, I, T>(vals: I, power: u8) -> T
where
    I: IntoIterator<Item = &'a T>,
    &'a T: Pow<u8>,
    T: 'a + From<u32> + Sum<&'a T> + Sum<<&'a T as Pow<u8>>::Output> {
        let iter = vals.into_iter();
        match power {
            0 => T::from(iter.count() as u32), // Treating 0 to the 0th as 1
            1 => iter.sum(),
            _ => iter.map(|x| x.pow(power)).sum()
        }
}

pub fn mean<'a, I, T>(vals: I, power: u8) -> T
where
    I: IntoIterator<Item = &'a T> + Copy,
    &'a T: Pow<u8>,
    T: 'a + From<u32> + Div + From<<T as Div>::Output> + Sum<&'a T> + Sum<<&'a T as Pow<u8>>::Output> {
        let count = T::from(vals.into_iter().count() as u32);
        let sum = sum_pow(vals, power);
        T::from(sum / count)
}

pub fn rms<'a, I, T>(vals: I) -> f64
where
    I: IntoIterator<Item = &'a T> + Copy,
    &'a T: Pow<u8>,
    T: 'a + From<u32> + Div + From<<T as Div>::Output> + Sum<&'a T> + Sum<<&'a T as Pow<u8>>::Output>,
    f64: From<T> {
    let length = vals.into_iter().count() as f64;
    let sum = f64::from(sum_pow(vals, 2));
    let mean = sum / length;
    mean.sqrt()
}
