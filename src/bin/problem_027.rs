use itertools::Itertools;
use num::ToPrimitive;
use project_euler_solutions::factors::is_prime;
use std::i64;

// Quadratic primes
// Euler discovered the remarkable quadratic formula:
//
//          n² + n + 41
//
// It turns out that the formula will produce 40 primes for the consecutive integer values 0 <= n <= 39. However, when
// n = 40, 40² + 40 + 41 = 40($0 +1) is divisible by 41, and certainly when n = 41, 41² + 41 + 41 is clearly divisible by 41.
//
// The incredible formula n² -79n + 1601 was discovered, which produces 80 primes for the consecutive values 0 <= n <= 79. The
// product of the coefficients, −79 and 1601, is −126479.
//
// Considering quadratics of the form:
//
//          n² + an + b, where |a| < 1000 and |b| <= 1000
//
// where |n| is the modulus/absolute value of n
// e.g. |11| = 11 and |-4| = 4
//
// Find the product of the coefficients, a and b, for the quadratic expression that produces the maximum number of primes for
// consecutive values of m, starting with n = 0.

struct Remarkable {
    a: i64,
    b: i64,
    n: usize,
}

impl Iterator for Remarkable {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.n.to_i64().unwrap();
        self.n += 1;
        let result = n * n + self.a * n + self.b;
        return if is_prime(&result.abs().to_u64().unwrap()) {
            Some(result)
        } else {
            None
        };
    }
}

fn num_remarkable_primes(a: i64, b: i64) -> usize {
    return Remarkable { a, b, n: 0 }.count();
}

fn problem_027() -> ((i64, i64), usize) {
    return (-999..=999)
        .cartesian_product(-1000..=1000)
        .map(|(a, b)| ((a, b), num_remarkable_primes(a, b)))
        .max_by_key(|i| i.1)
        .unwrap();
}

fn main() {
    println!("a=1 b=41 gives {} primes", num_remarkable_primes(1, 41));
    println!(
        "a=-79 b=1601 gives {} primes",
        num_remarkable_primes(-79, 1601)
    );

    let ((a, b), primes) = problem_027();

    println!("a={} b ={} gives {} primes", a, b, primes);
    println!("a * b = {}", a * b);
}

#[cfg(test)]
mod tests {
    use crate::{num_remarkable_primes, problem_027};

    #[test]
    fn test_num_remarkable_primes_1_41() {
        assert_eq!(num_remarkable_primes(1, 41), 40);
    }

    #[test]
    fn test_num_remarkable_primes_neg79_1601() {
        assert_eq!(num_remarkable_primes(-79, 1601), 80);
    }

    #[test]
    fn test_problem_027() {
        let (a, b) = problem_027().0;
        assert_eq!(a * b, -59231);
    }
}
