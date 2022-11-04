use num::ToPrimitive;
use project_euler_solutions::factors::is_prime;

// Circular primes
// The number, 197, is called a circular prime because all rotations of the digits: 197, 971, and 719, are themselves prime.
//
// There are thirteen such primes below 100: 2, 3, 5, 7, 11, 13, 17, 31, 37, 71, 73, 79, and 97.
//
// How many circular primes are there below one million?

struct RotDigits {
    next: Option<u32>,
    first: u32,
    pow_ten: u32,
}

impl Iterator for RotDigits {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.next;
        match ret {
            None => (),
            Some(cur) => {
                let next = (cur % 10) * self.pow_ten + cur / 10;
                self.next = if next == self.first { None } else { Some(next) };
            }
        };
        ret
    }
}

fn rotate_digits(i: u32) -> RotDigits {
    RotDigits {
        first: i,
        next: Some(i),
        // Can't do log of integer (https://github.com/rust-lang/rust/issues/70887) so get the len of the string repr
        pow_ten: 10u32.pow(i.to_string().len().to_u32().unwrap() - 1),
    }
}

fn is_circular_prime(i: &u32) -> bool {
    rotate_digits(*i).all(|x| is_prime(&x.to_u64().unwrap()))
}

#[cfg(test)]
fn problem_035_example() -> usize {
    (1..100).filter(is_circular_prime).count()
}

fn problem_035() -> usize {
    (1..1000000).filter(is_circular_prime).count()
}

pub fn main() {
    println!("{}", problem_035());
}

#[cfg(test)]
mod tests {
    use crate::{problem_035, problem_035_example};

    #[test]
    fn test_problem_035_example() {
        let result = problem_035_example();
        assert_eq!(result, 13);
    }

    #[test]
    fn test_problem_035() {
        let result = problem_035();
        assert_eq!(result, 55);
    }
}
