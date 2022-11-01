use project_euler_solutions::factors::is_prime;

// Summation of primes
// The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
// Find the sum of all the primes below two million.

fn sum_primes_below_n(n: u64) -> u64 {
    let mut sum = 0;
    let mut i = 0;
    while i < n {
        if is_prime(&i) {
            sum += i;
        }
        i += if i > 2 { 2 } else { 1 };
    }
    return sum;
}

#[cfg(test)]
fn problem_010_example() -> u64 {
    sum_primes_below_n(10)
}

fn problem_010() -> u64 {
    sum_primes_below_n(2000000)
}

fn main() {
    println!("{}", problem_010())
}

#[cfg(test)]
mod tests {
    use crate::{problem_010, problem_010_example};

    #[test]
    fn test_problem_010_example() {
        let result = problem_010_example();
        assert_eq!(result, 17);
    }

    #[test]
    fn test_problem_010() {
        let result = problem_010();
        assert_eq!(result, 142913828922);
    }
}
