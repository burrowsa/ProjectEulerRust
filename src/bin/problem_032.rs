use itertools::Itertools;
use project_euler_solutions::factors::factors;

// Pandigital products
// We shall say that an n-digit number is pandigital if it makes use of all the digits 1 to n exactly once; for example, the 5-digit number, 15234, is 1 through 5 pandigital.
//
// The product 7254 is unusual, as the identity, 39 × 186 = 7254, containing multiplicand, multiplier, and product is 1 through 9 pandigital.
//
// Find the sum of all products whose multiplicand/multiplier/product identity can be written as a 1 through 9 pandigital.
//
// HINT: Some products can be obtained in more than one way so be sure to only include it once in your sum.

fn problem_032() -> u64 {
    (4000..8000)
        .filter(|n| {
            factors(n).any(|(i, j)| {
                i.to_string()
                    .chars()
                    .chain(j.to_string().chars())
                    .chain(n.to_string().chars())
                    .sorted()
                    .collect::<String>()
                    == "123456789"
            })
        })
        .sum::<u64>()
}

fn main() {
    println!("{}", problem_032());
}

#[cfg(test)]
mod tests {
    use crate::problem_032;

    #[test]
    fn test_problem_032() {
        let result = problem_032();
        assert_eq!(result, 45228);
    }
}
