use itertools::Itertools;
use num::ToPrimitive;
use num_traits::identities::{One, Zero};
use num_traits::NumOps;
use project_euler_solutions::series::ap::sum_n_terms_ap;
use std::collections::HashSet;

// Multiples of 3 or 5
// If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
// Find the sum of all the multiples of 3 or 5 below 1000.

#[cfg(test)]
fn brute_force(n: u32, factors: &[u32]) -> u32 {
    let mut sum = 0;

    for i in 1..n {
        for j in factors {
            if i % j == 0 {
                sum += i;
                break;
            }
        }
    }
    sum
}

fn sum_multiples_of_i_less_than_n<T>(n: T, i: T) -> T
where
    T: NumOps + Copy + One + Zero,
{
    sum_n_terms_ap(&((n - T::one()) / i + T::one()), &T::zero(), &i)
}

fn remove_multiples(numbers: &[u32]) -> Vec<u32> {
    let mut out = Vec::new();
    for i in numbers.iter().sorted() {
        let mut add = true;
        for j in &out {
            add = add && (i % *j) != 0;
        }
        if add {
            out.push(*i);
        }
    }
    out
}

fn combined_factors(numbers: &[u32]) -> HashSet<i64> {
    let mut out = HashSet::new();

    let simplified = remove_multiples(numbers);

    for i in 1..=simplified.len() {
        for c in simplified.iter().combinations(i) {
            let mut product = -1;
            for j in c {
                product *= -j.to_i64().unwrap();
            }
            out.insert(product);
        }
    }

    out
}

fn algebraic_solution(n: u32, factors: &[u32]) -> u32 {
    let mut sum = 0;
    for i in combined_factors(factors) {
        sum += i.signum()
            * sum_multiples_of_i_less_than_n(n, i.abs().to_u32().unwrap())
                .to_i64()
                .unwrap();
    }
    sum.to_u32().unwrap()
}

#[cfg(test)]
fn problem_001_example() -> u32 {
    let n = 10;
    let factors = [3, 5];
    algebraic_solution(n, &factors)
}

#[cfg(test)]
fn problem_001_brute_example() -> u32 {
    let n = 10;
    let factors = [3, 5];
    brute_force(n, &factors)
}

fn problem_001() -> u32 {
    let n = 1000;
    let factors = [3, 5];
    algebraic_solution(n, &factors)
}

#[cfg(test)]
fn problem_001_brute() -> u32 {
    let n = 1000;
    let factors = [3, 5];
    brute_force(n, &factors)
}

fn main() {
    println!("{}", problem_001());
}

#[cfg(test)]
mod tests {
    use crate::{problem_001, problem_001_brute, problem_001_brute_example, problem_001_example};

    #[test]
    fn test_problem_001_example_brute() {
        let result = problem_001_brute_example();
        assert_eq!(result, 23);
    }

    #[test]
    fn test_problem_001_example() {
        let result = problem_001_example();
        assert_eq!(result, 23);
    }

    #[test]
    fn test_problem_001_brute() {
        let result = problem_001_brute();
        assert_eq!(result, 233168);
    }
    #[test]
    fn test_problem_001() {
        let result = problem_001();
        assert_eq!(result, 233168);
    }
}
