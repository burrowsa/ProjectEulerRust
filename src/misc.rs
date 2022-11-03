use std::{fmt::Debug, iter::Sum, str::FromStr};

pub struct Digits {
    i: u64,
}

impl Iterator for Digits {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        return if self.i != 0 {
            let ret = self.i % 10;
            self.i = self.i / 10;
            Some(ret)
        } else {
            None
        };
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
    return i
        .to_string()
        .chars()
        .map(|x| x.to_string().parse::<O>().unwrap())
        .sum::<O>();
}

#[cfg(test)]
mod tests {
    use super::{digits, sum_digits};

    #[test]
    fn digits_0() {
        let items: Vec<u64> = digits(0).collect();
        assert_eq!(items, vec![])
    }

    #[test]
    fn digits_1() {
        let items: Vec<u64> = digits(1).collect();
        assert_eq!(items, vec![1])
    }

    #[test]
    fn digits_123() {
        let items: Vec<u64> = digits(123).collect();
        assert_eq!(items, vec![3, 2, 1])
    }

    #[test]
    fn digits_105() {
        let items: Vec<u64> = digits(105).collect();
        assert_eq!(items, vec![5, 0, 1])
    }

    #[test]
    fn sum_digits_0() {
        let items: u64 = sum_digits(0);
        assert_eq!(items, 0)
    }

    #[test]
    fn sum_digits_1() {
        let items: u64 = sum_digits(1);
        assert_eq!(items, 1)
    }

    #[test]
    fn sum_digits_123() {
        let items: u64 = sum_digits(123);
        assert_eq!(items, 6)
    }

    #[test]
    fn sum_digits_105() {
        let items: u64 = sum_digits(105);
        assert_eq!(items, 6)
    }
}
