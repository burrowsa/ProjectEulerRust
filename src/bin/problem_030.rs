// Digit fifth powers
// Surprisingly there are only three numbers that can be written as the sum of fourth powers of their digits:
//
// 1634 = 1^4 + 6^4 + 3^4 + 4^4
// 8208 = 8^4 + 2^4 + 0^4 + 8^4
// 9474 = 9^4 + 4^4 + 7^4 + 4^4
// As 1 = 1^4 is not a sum it is not included.
//
// The sum of these numbers is 1634 + 8208 + 9474 = 19316.
//
// Find the sum of all the numbers that can be written as the sum of fifth powers of their digits.

fn nth_digit_powers(n: u32) -> u32 {
    (2..999999u32)
        .filter(|i| {
            *i == ((i % 10).pow(n)
                + ((i / 10) % 10).pow(n)
                + ((i / 100) % 10).pow(n)
                + ((i / 1000) % 10).pow(n)
                + ((i / 10000) % 10).pow(n)
                + ((i / 100000) % 10).pow(n)
                + ((i / 1000000) % 10).pow(n))
        })
        .sum::<u32>()
}

#[cfg(test)]
fn problem_030_example() -> u32 {
    nth_digit_powers(4)
}

fn problem_030() -> u32 {
    nth_digit_powers(5)
}

fn main() {
    println!("{}", problem_030());
}

#[cfg(test)]
mod tests {
    use crate::{problem_030, problem_030_example};

    #[test]
    fn test_problem_030_example() {
        let result = problem_030_example();
        assert_eq!(result, 19316);
    }

    #[test]
    fn test_problem_030() {
        let result = problem_030();
        assert_eq!(result, 443839);
    }
}
