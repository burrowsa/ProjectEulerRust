use num_traits::NumOps;
use num_traits::identities::One;

pub fn sum_n_terms_ap<T>(n: &T, a: &T, d: &T) -> T
where
    T: NumOps + One + Copy
{
    return (*n) * ((T::one() + T::one()) * (*a) + ((*n) - T::one()) * (*d)) / (T::one() + T::one());
}
