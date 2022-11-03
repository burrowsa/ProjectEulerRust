use num_traits::identities::One;
use num_traits::NumOps;

pub fn sum_n_terms_ap<T>(n: &T, a: &T, d: &T) -> T
where
    T: NumOps + One + Copy,
{
    return (*n) * ((T::one() + T::one()) * (*a) + ((*n) - T::one()) * (*d))
        / (T::one() + T::one());
}

#[cfg(test)]
mod tests {
    use super::sum_n_terms_ap;

    #[test]
    fn sum_ap_1_1_1() {
        let result = sum_n_terms_ap(&1, &1, &1);
        assert_eq!(result, 1);
    }

    #[test]
    fn sum_ap_2_1_1() {
        let result = sum_n_terms_ap(&2, &1, &1);
        assert_eq!(result, 3);
    }

    #[test]
    fn sum_ap_3_1_1() {
        let result = sum_n_terms_ap(&3, &1, &1);
        assert_eq!(result, 6);
    }

    #[test]
    fn sum_ap_1_2_0() {
        let result = sum_n_terms_ap(&1, &1, &1);
        assert_eq!(result, 1);
    }

    #[test]
    fn sum_ap_2_2_0() {
        let result = sum_n_terms_ap(&2, &1, &1);
        assert_eq!(result, 3);
    }

    #[test]
    fn sum_ap_3_2_0() {
        let result = sum_n_terms_ap(&3, &1, &1);
        assert_eq!(result, 6);
    }

    #[test]
    fn sum_ap_10_1_1() {
        let result = sum_n_terms_ap(&10, &1, &1);
        assert_eq!(result, 55);
    }

    #[test]
    fn sum_ap_10_2_0() {
        let result = sum_n_terms_ap(&10, &1, &1);
        assert_eq!(result, 55);
    }
}
