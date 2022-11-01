use project_euler_solutions::factors::is_prime;

// 10001st prime
// By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.
// What is the 10 001st prime number?

fn nth_prime(n: usize) -> u64 {
    let mut n_primes = 0;
    let mut i = 1;
    while n_primes < n {
        i += if i > 2 { 2 } else { 1 };
        if is_prime(&i) {
            n_primes += 1;
        }
    }
    return i;
}

#[cfg(test)]
fn problem_007_example() -> u64 {
    nth_prime(6)
}

fn problem_007() -> u64 {
    nth_prime(10001)
}

fn main() {
    println!("{}", problem_007())
}

#[cfg(test)]
mod tests {
    use crate::{problem_007, problem_007_example};

    #[test]
    fn test_problem_007_example() {
        let result = problem_007_example();
        assert_eq!(result, 13);
    }

    #[test]
    fn test_problem_007() {
        let result = problem_007();
        assert_eq!(result, 104743);
    }
}
