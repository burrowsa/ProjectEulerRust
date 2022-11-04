use project_euler_solutions::numbers::int_to_words;

// Number letter counts
// If the numbers 1 to 5 are written out in words: one, two, three, four, five, then there are
// 3 + 3 + 5 + 4 + 4 = 19 lettersused in total.
//
// If all the numbers from 1 to 1000 (one thousand) inclusive were written out in words, how many
// letters would be used?
//
// NOTE: Do not count spaces or hyphens. For example, 342 (three hundred and forty-two) contains 23
// letters and 115 (one hundred and fifteen) contains 20 letters. The use of "and" when writing out
// numbers is in compliance with British usage.

fn number_letter_counts(n: usize) -> usize {
    (1..=n)
        .map(|i| int_to_words(i).replace(' ', "").replace('-', "").len())
        .sum::<usize>()
}

#[cfg(test)]
fn problem_017_example() -> usize {
    number_letter_counts(5)
}

fn problem_017() -> usize {
    number_letter_counts(1000)
}

fn main() {
    println!("{}", problem_017());
}

#[cfg(test)]
mod tests {
    use crate::{problem_017, problem_017_example};

    #[test]
    fn test_problem_017_example() {
        let result = problem_017_example();
        assert_eq!(result, 19);
    }

    #[test]
    fn test_problem_017() {
        let result = problem_017();
        assert_eq!(result, 21124);
    }
}
