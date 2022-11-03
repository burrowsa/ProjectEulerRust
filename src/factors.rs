use itertools::Itertools;
use memoize::memoize;

pub struct Factors {
    n: u64,
    next: u64,
}

impl Iterator for Factors {
    type Item = (u64, u64);

    fn next(&mut self) -> Option<Self::Item> {
        let mx = (self.n as f64).sqrt() as u64 + 1;
        while self.next <= mx {
            if self.n % self.next == 0 {
                let result = (self.next, self.n / self.next);
                if result.0 > result.1 {
                    break;
                }
                self.next += 1;
                return Some(result);
            }
            self.next += 1;
        }
        return None;
    }
}

pub fn factors(n: &u64) -> Factors {
    Factors { n: *n, next: 1 }
}

pub fn is_prime(i: &u64) -> bool {
    return i != &1 && factors(i).count() == 1;
}

#[memoize]
pub fn sum_proper_divisors(n: u64) -> u64 {
    return factors(&n)
        .map(|(i, _)| i)
        .chain(factors(&n).map(|(_, j)| j))
        .unique()
        .sum::<u64>()
        - n;
}

pub fn is_perfect(n: &u64) -> bool {
    return sum_proper_divisors(*n) == *n;
}

pub fn is_deficient(n: &u64) -> bool {
    return sum_proper_divisors(*n) < *n;
}

pub fn is_abundant(n: &u64) -> bool {
    return sum_proper_divisors(*n) > *n;
}

#[cfg(test)]
mod tests {
    use super::{factors, is_prime};

    #[test]
    fn factors_1() {
        let items: Vec<(u64, u64)> = factors(&1).collect();
        assert_eq!(items, vec![(1, 1)])
    }

    #[test]
    fn factors_2() {
        let items: Vec<(u64, u64)> = factors(&2).collect();
        assert_eq!(items, vec![(1, 2)])
    }

    #[test]
    fn factors_4() {
        let items: Vec<(u64, u64)> = factors(&4).collect();
        assert_eq!(items, vec![(1, 4), (2, 2)])
    }

    #[test]
    fn factors_16() {
        let items: Vec<(u64, u64)> = factors(&16).collect();
        assert_eq!(items, vec![(1, 16), (2, 8), (4, 4)])
    }

    #[test]
    fn is_prime_1() {
        assert_eq!(is_prime(&1), false)
    }

    #[test]
    fn is_prime_2() {
        assert_eq!(is_prime(&2), true)
    }

    #[test]
    fn is_prime_3() {
        assert_eq!(is_prime(&3), true)
    }

    #[test]
    fn is_prime_4() {
        assert_eq!(is_prime(&4), false)
    }

    #[test]
    fn is_prime_5() {
        assert_eq!(is_prime(&5), true)
    }

    #[test]
    fn is_prime_6() {
        assert_eq!(is_prime(&6), false)
    }
}
