use project_euler_solutions::series::ap::sum_n_terms_ap;
use project_euler_solutions::series::squares::sum_first_n_squares;

// Sum square difference
// The sum of the squares of the first ten natural numbers is,
//
//              1² + 2² + ... + 10² = 385
//
// The square of the sum of the first ten natural numbers is,
//
//              (1 + 2 + ... + 10)² = 3025
//
// Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is 3025 - 385 = 2640.
//
// Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.

#[cfg(test)]
fn brute_force(n: i64) -> (i64, i64, i64) {
    let sum_nums_sq = (1..=n).sum::<i64>().pow(2);
    let sum_sqs = (1..=n).map(|i| i * i).sum::<i64>();
    (sum_nums_sq, sum_sqs, sum_nums_sq - sum_sqs)
}

fn algebraic_solution(n: i64) -> (i64, i64, i64) {
    let sum_nums = sum_n_terms_ap(&n, &1, &1);
    let sum_nums_sq = sum_nums * sum_nums;
    let sum_sqs = sum_first_n_squares(n);
    (sum_nums_sq, sum_sqs, sum_nums_sq - sum_sqs)
}

#[cfg(test)]
fn problem_006() -> i64 {
    algebraic_solution(100).2
}

#[cfg(test)]
fn problem_006_brute() -> i64 {
    brute_force(100).2
}

#[cfg(test)]
fn problem_006_example() -> i64 {
    algebraic_solution(10).2
}

#[cfg(test)]
fn problem_006_brute_example() -> i64 {
    brute_force(10).2
}

fn main() {
    let (sum_nums_sq, sum_sqs, sum_nums_sq_minus_sum_sqs) = algebraic_solution(100);
    println!(
        "{} + {} = {}",
        sum_nums_sq, sum_sqs, sum_nums_sq_minus_sum_sqs
    );
}

#[cfg(test)]
mod tests {
    use crate::{problem_006, problem_006_brute, problem_006_brute_example, problem_006_example};

    #[test]
    fn test_problem_006_example_brute() {
        let result = problem_006_brute_example();
        assert_eq!(result, 2640);
    }

    #[test]
    fn test_problem_006_example() {
        let result = problem_006_example();
        assert_eq!(result, 2640);
    }

    #[test]
    fn test_problem_006_brute() {
        let result = problem_006_brute();
        assert_eq!(result, 25164150);
    }

    #[test]
    fn test_problem_006() {
        let result = problem_006();
        assert_eq!(result, 25164150);
    }
}
