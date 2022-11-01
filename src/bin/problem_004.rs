// Largest palindrome product
// A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.
// Find the largest palindrome made from the product of two 3-digit numbers.

use itertools::Itertools;
use std::ops::RangeInclusive;

fn is_palindrome(i: u32) -> bool {
    let s = i.to_string();
    let r = s.chars().rev().collect::<String>();
    return s == r;
}

fn largest_palindromic_product(r: RangeInclusive<u32>) -> (u32, u32) {
    (r.clone())
        .rev()
        .cartesian_product(r.rev())
        .filter(|(i, j)| (i > j) && is_palindrome(i * j))
        .next()
        .unwrap()
}

#[cfg(test)]
fn problem_004_example() -> (u32, u32) {
    largest_palindromic_product(90..=99)
}

fn problem_004() -> (u32, u32) {
    largest_palindromic_product(900..=999)
}

fn main() {
    let (i, j) = problem_004();
    println!("{} = {} x {}", i * j, i, j);
}

#[cfg(test)]
mod tests {
    use crate::{problem_004, problem_004_example};

    #[test]
    fn test_problem_004_example() {
        let (i, j) = problem_004_example();
        assert_eq!(i * j, 9009);
    }

    #[test]
    fn test_problem_004() {
        let (i, j) = problem_004();
        assert_eq!(i * j, 906609);
    }
}
