use project_euler_solutions::factors::sum_proper_divisors;

// Amicable numbers
// Let d(n) be defined as the sum of proper divisors of n (numbers less than n which divide evenly into n).
// If d(a) = b and d(b) = a, where a ≠ b, then a and b are an amicable pair and each of a and b are called amicable numbers.
//
// For example, the proper divisors of 220 are 1, 2, 4, 5, 10, 11, 20, 22, 44, 55 and 110; therefore d(220) = 284.
// The proper divisors of 284 are 1, 2, 4, 71 and 142; so d(284) = 220.
//
// Evaluate the sum of all the amicable numbers under 10000.

fn is_amicable(n: &u64) -> bool {
    let sum_proper_divisors_n = sum_proper_divisors(*n);
    sum_proper_divisors_n != *n && sum_proper_divisors(sum_proper_divisors_n) == *n
}

fn problem_021() -> u64 {
    (1..10000).filter(is_amicable).sum::<u64>()
}

fn main() {
    println!("{}", problem_021());
}

#[cfg(test)]
mod tests {
    use crate::problem_021;

    #[test]
    fn test_problem_021() {
        let result = problem_021();
        assert_eq!(result, 31626);
    }
}
