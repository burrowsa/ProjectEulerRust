use num::BigUint;
use num_bigint::ToBigUint;
use project_euler_solutions::series::fib::fibonacci;

// 1000-digit Fibonacci number
// The Fibonacci sequence is defined by the recurrence relation:
//
// Fn = Fn−1 + Fn−2, where F1 = 1 and F2 = 1.
// Hence the first 12 terms will be:
//
// F1 = 1
// F2 = 1
// F3 = 2
// F4 = 3
// F5 = 5
// F6 = 8
// F7 = 13
// F8 = 21
// F9 = 34
// F10 = 55
// F11 = 89
// F12 = 144
// The 12th term, F12, is the first term to contain three digits.
//
// What is the index of the first term in the Fibonacci sequence to contain 1000 digits?

fn index_of_first_n_digit_term(n: u32) -> usize {
    let digits = 10.to_biguint().unwrap().pow(n - 1);
    return fibonacci::<BigUint>()
        .enumerate()
        .find(|(_, x)| *x >= digits)
        .unwrap()
        .0;
}

#[cfg(test)]
fn problem_025_example() -> usize {
    index_of_first_n_digit_term(3)
}

fn problem_025() -> usize {
    index_of_first_n_digit_term(1000)
}

fn main() {
    println!("{}", problem_025());
}

#[cfg(test)]
mod tests {
    use crate::{problem_025, problem_025_example};

    #[test]
    fn test_problem_025_example() {
        let result = problem_025_example();
        assert_eq!(result, 12);
    }

    #[test]
    fn test_problem_025() {
        let result = problem_025();
        assert_eq!(result, 4782);
    }
}
