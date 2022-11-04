use std::fs;

use itertools::Itertools;
use num::ToPrimitive;

// Names scores
// Using names.txt (in resources folder), a 46K text file containing over five-thousand first names, begin by sorting it into alphabetical
// order. Then working out the alphabetical value for each name, multiply this value by its alphabetical position in the list to obtain a
// name score.

// For example, when the list is sorted into alphabetical order, COLIN, which is worth 3 + 15 + 12 + 9 + 14 = 53, is the 938th name in the
// list. So, COLIN would obtain a score of 938 × 53 = 49714.

// What is the total of all the name scores in the file?

fn problem_022() -> u64 {
    let file_path = "resources/problem_22_names.txt";
    let contents = fs::read_to_string(file_path).unwrap();

    return contents
        .split(',')
        .map(|i| i.strip_prefix('\"').unwrap().strip_suffix('\"').unwrap())
        .sorted()
        .enumerate()
        .map(|(i, name)| {
            (1 + i).to_u64().unwrap()
                * name
                    .bytes()
                    .map(|c| (c - 64u8).to_u64().unwrap())
                    .sum::<u64>()
        })
        .sum::<u64>();
}

fn main() {
    println!("{}", problem_022());
}

#[cfg(test)]
mod tests {
    use crate::problem_022;

    #[test]
    fn test_problem_022() {
        let result = problem_022();
        assert_eq!(result, 871198282);
    }
}
