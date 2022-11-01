extern crate num;
use num::BigUint;
use num_bigint::ToBigUint;
use project_euler_solutions::misc::sum_digits;

// Factorial digit sum
// n! means n × (n − 1) × ... × 3 × 2 × 1
//
// For example, 10! = 10 × 9 × ... × 3 × 2 × 1 = 3628800,
// and the sum of the digits in the number 10! is 3 + 6 + 2 + 8 + 8 + 0 + 0 = 27.
//
// Find the sum of the digits in the number 100!

fn factorial_digit_sum(n: u32) -> u32 {
    let mut result = n.to_biguint().unwrap();
    for i in 1..n {
        result *= i.to_biguint().unwrap();
    }

    return sum_digits::<BigUint, u32>(result);
}

#[cfg(test)]
fn problem_020_example() -> u32 {
    factorial_digit_sum(10)
}

fn problem_020() -> u32 {
    factorial_digit_sum(100)
}

fn main() {
    print!("{}", problem_020());
}

#[cfg(test)]
mod tests {
    use crate::{problem_020, problem_020_example};

    #[test]
    fn test_problem_020_example() {
        let result = problem_020_example();
        assert_eq!(result, 27);
    }

    #[test]
    fn test_problem_020() {
        let result = problem_020();
        assert_eq!(result, 648);
    }
}
