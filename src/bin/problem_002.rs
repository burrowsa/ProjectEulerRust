// Even Fibonacci numbers
// Each new term in the Fibonacci sequence is generated by adding the previous two terms. By starting with 1 and 2, the first 10 terms will be:
//
//                1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...
//
// By considering the terms in the Fibonacci sequence whose values do not exceed four million, find the sum of the even-valued terms.

#[cfg(test)]
fn brute_force(mx: u32) -> u32 {
    use project_euler_solutions::series::fib::fibonacci;

    fibonacci::<u32>()
        .skip(2)
        .take_while(|i| i <= &mx)
        .filter(|i| i % 2 == 0)
        .sum()
}

struct EvenFibonacci {
    curr: u32,
    next: u32,
}

impl Iterator for EvenFibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.curr;

        self.curr = self.next;
        self.next = current + 4 * self.next;

        Some(current)
    }
}

fn even_fibonacci() -> EvenFibonacci {
    EvenFibonacci { curr: 0, next: 2 }
}

fn three_times_faster(mx: u32) -> u32 {
    even_fibonacci().take_while(|i| i <= &mx).sum()
}

#[cfg(test)]
fn problem_002_brute_example() -> u32 {
    let mx = 90;
    brute_force(mx)
}

#[cfg(test)]
fn problem_002_brute() -> u32 {
    let mx = 4000000;
    brute_force(mx)
}

#[cfg(test)]
fn problem_002_example() -> u32 {
    let mx = 90;
    three_times_faster(mx)
}

fn problem_002() -> u32 {
    let mx = 4000000;
    three_times_faster(mx)
}

fn main() {
    println!("{}", problem_002());
}

#[cfg(test)]
mod tests {
    use crate::{problem_002, problem_002_brute, problem_002_brute_example, problem_002_example};

    #[test]
    fn test_problem_002_example_brute() {
        let result = problem_002_brute_example();
        assert_eq!(result, 44);
    }

    #[test]
    fn test_problem_002_example() {
        let result = problem_002_example();
        assert_eq!(result, 44);
    }

    #[test]
    fn test_problem_002_brute() {
        let result = problem_002_brute();
        assert_eq!(result, 4613732);
    }
    #[test]
    fn test_problem_002() {
        let result = problem_002();
        assert_eq!(result, 4613732);
    }
}
