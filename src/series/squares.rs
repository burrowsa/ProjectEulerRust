pub fn sum_first_n_squares(n: i64) -> i64 {
    return n * (n + 1) * (2 * n + 1) / 6;
}

#[cfg(test)]
mod tests {
    use super::sum_first_n_squares;

    #[test]
    fn sum_first_0_squares() {
        assert_eq!(sum_first_n_squares(0), 0)
    }

    #[test]
    fn sum_first_1_squares() {
        assert_eq!(sum_first_n_squares(1), 1)
    }

    #[test]
    fn sum_first_2_squares() {
        assert_eq!(sum_first_n_squares(2), 1 + 4)
    }

    #[test]
    fn sum_first_3_squares() {
        assert_eq!(sum_first_n_squares(3), 1 + 4 + 9)
    }

    #[test]
    fn sum_first_4_squares() {
        assert_eq!(sum_first_n_squares(4), 1 + 4 + 9 + 16)
    }
}
