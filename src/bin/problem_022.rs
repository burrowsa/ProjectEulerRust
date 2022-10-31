use std::fs;

use itertools::Itertools;
use num::ToPrimitive;

fn main() {
    let file_path = "resources/problem_22_names.txt";
    let contents = fs::read_to_string(file_path).unwrap();

    let sum_scores = contents
        .split(",")
        .map(|i| i.strip_prefix("\"").unwrap().strip_suffix("\"").unwrap())
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
    println!("{} == 871198282", sum_scores);
}
