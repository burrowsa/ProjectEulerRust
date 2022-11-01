use itertools::Itertools;
use project_euler_solutions::factors::is_abundant;

// Non-abundant sums
// A perfect number is a number for which the sum of its proper divisors is exactly equal to the number.
// For example, the sum of the proper divisors of 28 would be 1 + 2 + 4 + 7 + 14 = 28, which means that
// 28 is a perfect number.
//
// A number n is called deficient if the sum of its proper divisors is less than n and it is called abundant
// if this sum exceeds n.
//
// As 12 is the smallest abundant number, 1 + 2 + 3 + 4 + 6 = 16, the smallest number that can be written as
// the sum of two abundant numbers is 24. By mathematical analysis, it can be shown that all integers greater
// than 28123 can be written as the sum of two abundant numbers. However, this upper limit cannot be reduced
// any further by analysis even though it is known that the greatest number that cannot be expressed as the
// sum of two abundant numbers is less than this limit.
//
// Find the sum of all the positive integers which cannot be written as the sum of two abundant numbers.

fn problem_023() -> u64 {
    let abundant_numbers: Vec<u64> = (1..28124).filter(is_abundant).collect();

    let sum_numbers_sum_of_two_abundant_numbers = abundant_numbers
        .iter()
        .cartesian_product(abundant_numbers.iter())
        .map(|(i, j)| i + j)
        .filter(|x| x <= &28123)
        .unique()
        .sum::<u64>();

    return (1..28124).sum::<u64>() - sum_numbers_sum_of_two_abundant_numbers;
}

fn main() {
    println!("{}", problem_023());
}

#[cfg(test)]
mod tests {
    use crate::problem_023;

    #[test]
    fn test_problem_023() {
        let result = problem_023();
        assert_eq!(result, 4179871);
    }
}
