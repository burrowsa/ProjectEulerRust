use itertools::Itertools;

// Lexicographic permutations
// A permutation is an ordered arrangement of objects. For example, 3124 is one possible permutation of the digits
// 1, 2, 3 and 4. If all of the permutations are listed numerically or alphabetically, we call it lexicographic order.
// The lexicographic permutations of 0, 1 and 2 are:
//
//       012   021   102   120   201   210
//
// What is the millionth lexicographic permutation of the digits 0, 1, 2, 3, 4, 5, 6, 7, 8 and 9?

fn problem_024() -> u64 {
    let millionth_perm = (0..10).permutations(10).nth(1000000 - 1).unwrap();
    return millionth_perm
        .iter()
        .map(|i| i.to_string())
        .join("")
        .parse::<u64>()
        .unwrap();
}

fn main() {
    println!("{}", problem_024());
}

#[cfg(test)]
mod tests {
    use crate::problem_024;

    #[test]
    fn test_problem_024() {
        let result = problem_024();
        assert_eq!(result, 2783915460);
    }
}
