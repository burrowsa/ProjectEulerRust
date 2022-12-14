use itertools::Itertools;
use num_bigint::ToBigUint;

// Distinct powers
// Show HTML problem content
// Problem 29
// Consider all integer combinations of ab for 2 ≤ a ≤ 5 and 2 ≤ b ≤ 5:
//
//           2^2=4, 2^3=8, 2^4=16, 2^5=32
//           3^2=9, 3^3=27, 3^4=81, 3^5=243
//           4^2=16, 4^3=64, 4^4=256, 4^5=1024
//           5^2=25, 5^3=125, 5^4=625, 5^5=3125
//
// If they are then placed in numerical order, with any repeats removed, we get the following sequence of 15 distinct terms:
//
// 4, 8, 9, 16, 25, 27, 32, 64, 81, 125, 243, 256, 625, 1024, 3125
//
// How many distinct terms are in the sequence generated by ab for 2 ≤ a ≤ 100 and 2 ≤ b ≤ 100?

fn distinct_powers(n: u32) -> usize {
    (2..=n)
        .cartesian_product(2..=n)
        .map(|(a, b)| a.to_biguint().unwrap().pow(b))
        .sorted()
        .unique()
        .count()
}

#[cfg(test)]
fn problem_029_example() -> usize {
    distinct_powers(5)
}
fn problem_029() -> usize {
    distinct_powers(100)
}

fn main() {
    println!("{}", problem_029())
}

#[cfg(test)]
mod tests {
    use crate::{problem_029, problem_029_example};

    #[test]
    fn test_problem_029_example() {
        let result = problem_029_example();
        assert_eq!(result, 15);
    }

    #[test]
    fn test_problem_029() {
        let result = problem_029();
        assert_eq!(result, 9183);
    }
}
