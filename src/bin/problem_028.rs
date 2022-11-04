// Starting with the number 1 and moving to the right in a clockwise direction a 5 by 5 spiral is formed as follows:
//
//             [21]22 23 24[25]
//              20 [7] 8 [9]10
//              19  6 [1] 2 11
//              18 [5] 4 [3]12
//             [17]16 15 14[13]
//
// It can be verified that the sum of the numbers on the diagonals is 101.
//
// What is the sum of the numbers on the diagonals in a 1001 by 1001 spiral formed in the same way?

struct Spiral {
    n: u32,
    diagonals: u32,
    step: u32,
    size: u32,
}

impl Iterator for Spiral {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.n;
        if self.diagonals == 4 {
            self.diagonals = 0;
            self.step += 2;
        }
        self.n += self.step;
        self.diagonals += 1;
        if n <= self.size * self.size {
            Some(n)
        } else {
            None
        }
    }
}

#[cfg(test)]
fn problem_028_example() -> u32 {
    Spiral {
        n: 1,
        size: 5,
        diagonals: 0,
        step: 2,
    }
    .sum()
}

fn problem_028() -> u32 {
    Spiral {
        n: 1,
        size: 1001,
        diagonals: 0,
        step: 2,
    }
    .sum()
}

fn main() {
    println!("{}", problem_028());
}

#[cfg(test)]
mod tests {
    use crate::{problem_028, problem_028_example};

    #[test]
    fn test_problem_028_example() {
        let result = problem_028_example();
        assert_eq!(result, 101);
    }

    #[test]
    fn test_problem_028() {
        let result = problem_028();
        assert_eq!(result, 669171001);
    }
}
