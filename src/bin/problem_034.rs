use project_euler_solutions::misc::digits;

// Digit factorials
// 145 is a curious number, as 1! + 4! + 5! = 1 + 24 + 120 = 145.
//
// Find the sum of all numbers which are equal to the sum of the factorial of their digits.
//
// Note: As 1! = 1 and 2! = 2 are not sums they are not included.

fn sum_fac_digits(n: u64) -> u64 {
    return digits(n).map(|i| (1..i + 1).product::<u64>()).sum();
}

#[cfg(test)]
fn problem_034_example() -> u64 {
    sum_fac_digits(145)
}

fn problem_034() -> u64 {
    (3..1000000).filter(|n| *n == sum_fac_digits(*n)).sum()
}

fn main() {
    println!("{}", problem_034());
}

#[cfg(test)]
mod tests {
    use crate::{problem_034, problem_034_example};

    #[test]
    fn test_problem_034_example() {
        let result = problem_034_example();
        assert_eq!(result, 145);
    }

    #[test]
    fn test_problem_034() {
        let result = problem_034();
        assert_eq!(result, 40730);
    }
}
