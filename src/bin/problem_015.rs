use memoize::memoize;

// Lattice paths
// Starting in the top left corner of a 2×2 grid, and only being able to move to the right and down,
// there are exactly 6 routes to the bottom right corner.
// How many such routes are there through a 20×20 grid?

#[memoize]
fn count_paths_impl(state: (i64, i64)) -> i64 {
    let (r, d) = state;
    if r == 0 || d == 0 {
        1
    } else {
        (if r > 0 {
            count_paths_impl((r - 1, d))
        } else {
            0
        } + if d > 0 {
            count_paths_impl((r, d - 1))
        } else {
            0
        })
    }
}

fn count_paths(side: i64) -> i64 {
    count_paths_impl((side, side))
}

#[cfg(test)]
fn problem_015_example() -> i64 {
    count_paths(2)
}

fn problem_015() -> i64 {
    count_paths(20)
}

fn main() {
    println!("{}", problem_015());
    // Note: Analytical answer possible from using https://en.wikipedia.org/wiki/Binomial_coefficient
}

#[cfg(test)]
mod tests {
    use crate::{problem_015, problem_015_example};

    #[test]
    fn test_problem_015_example() {
        let result = problem_015_example();
        assert_eq!(result, 6);
    }

    #[test]
    fn test_problem_015() {
        let result = problem_015();
        assert_eq!(result, 137846528820);
    }
}
