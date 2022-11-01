use project_euler_solutions::series::collatz::collatz;

// Longest Collatz sequence
// The following iterative sequence is defined for the set of positive integers:
//
// n → n/2 (n is even)
// n → 3n + 1 (n is odd)
//
// Using the rule above and starting with 13, we generate the following sequence:
//
// 13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1
// It can be seen that this sequence (starting at 13 and finishing at 1) contains 10 terms.
// Although it has not been proved yet (Collatz Problem), it is thought that all starting
// numbers finish at 1.
//
// Which starting number, under one million, produces the longest chain?
//
// NOTE: Once the chain starts the terms are allowed to go above one million.

fn longest_chain(n: usize) -> (usize, usize) {
    return (1..n)
        .map(|start| (start, collatz(start).count()))
        .max_by_key(|(_, chain_len)| *chain_len)
        .unwrap();
}

fn main() {
    let (mx_start, mx) = longest_chain(1000000);
    println!("{} {}", mx_start, mx);
}

#[cfg(test)]
mod tests {
    use crate::longest_chain;

    #[test]
    fn test_problem_014() {
        let (mx_start, mx) = longest_chain(1000000);
        assert_eq!(mx_start, 837799);
        assert_eq!(mx, 525);
    }
}
