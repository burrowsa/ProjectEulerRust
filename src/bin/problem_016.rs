use num::BigUint;
use num_bigint::ToBigUint;
use project_euler_solutions::misc::sum_digits;

// Power digit sum
// 2^15 = 32768 and the sum of its digits is 3 + 2 + 7 + 6 + 8 = 26.
// What is the sum of the digits of the number 2^1000?

fn sum_digits_two_to_power_n(n: u32) -> u32 {
    let two = 2u32.to_biguint().unwrap();
    let two_to_n = two.pow(n);
    return sum_digits::<BigUint, u32>(two_to_n);
}

#[cfg(test)]
fn problem_016_example() -> u32 {
    sum_digits_two_to_power_n(15)
}

fn problem_016() -> u32 {
    sum_digits_two_to_power_n(1000)
}

fn main() {
    println!("{}", problem_016());
}

#[cfg(test)]
mod tests {
    use crate::{problem_016, problem_016_example};

    #[test]
    fn test_problem_016_example() {
        let result = problem_016_example();
        assert_eq!(result, 26);
    }

    #[test]
    fn test_problem_016() {
        let result = problem_016();
        assert_eq!(result, 1366);
    }
}
