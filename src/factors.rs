use memoize::memoize;
use itertools::Itertools;

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
    return factors(&n).map(|(i, _)| i).chain(factors(&n).map(|(_, j)| j)).unique().sum::<u64>() - n;
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
