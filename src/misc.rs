use std::{fmt::Debug, iter::Sum, str::FromStr};

pub struct Digits {
    i: u64,
}

impl Iterator for Digits {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i != 0 {
            let ret = self.i % 10;
            self.i /= 10;
            Some(ret)
        } else {
            None
        }
    }
}

pub fn digits(i: u64) -> Digits {
    Digits { i }
}

pub fn sum_digits<T, O>(i: T) -> O
where
    T: ToString,
    O: Sum<O> + FromStr,
    <O as FromStr>::Err: Debug,
{
    i.to_string()
        .chars()
        .map(|x| x.to_string().parse::<O>().unwrap())
        .sum::<O>()
}

#[cfg(test)]
mod tests {
    use super::{digits, sum_digits};
    use test_case::test_case;

    #[test_case(0, vec![] ; "0")]
    #[test_case(1, vec![1] ; "1")]
    #[test_case(123, vec![3,2,1] ; "123")]
    #[test_case(105, vec![5,0,1] ; "105")]
    fn test_digits(i: u64, expected: Vec<u64>) {
        assert_eq!(digits(i).collect::<Vec<u64>>(), expected)
    }

    #[test_case(0, 0 ; "0")]
    #[test_case(1, 1 ; "1")]
    #[test_case(123, 6 ; "123")]
    #[test_case(105, 6 ; "105")]
    fn test_sum_digits(i: u64, expected: u64) {
        assert_eq!(sum_digits::<u64, u64>(i), expected)
    }
}
