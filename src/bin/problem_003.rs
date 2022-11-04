use project_euler_solutions::factors::{factors, is_prime};

// Largest prime factor
// The prime factors of 13195 are 5, 7, 13 and 29.
// What is the largest prime factor of the number 600851475143 ?

fn largest_prime_factor(n: &u64) -> u64 {
    factors(n).map(|(i, _)| i).filter(is_prime).max().unwrap()
}

#[cfg(test)]
fn problem_003_example() -> u64 {
    let n = 13195;
    largest_prime_factor(&n)
}

fn problem_003() -> u64 {
    let n = 600851475143;
    largest_prime_factor(&n)
}

fn main() {
    println!("{}", problem_003());
}

#[cfg(test)]
mod tests {
    use crate::{problem_003, problem_003_example};

    #[test]
    fn test_problem_003_example() {
        let result = problem_003_example();
        assert_eq!(result, 29);
    }

    #[test]
    fn test_problem_003() {
        let result = problem_003();
        assert_eq!(result, 6857);
    }
}
