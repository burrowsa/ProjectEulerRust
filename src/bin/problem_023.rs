use itertools::Itertools;
use project_euler_solutions::factors::is_abundant;

fn main() {
    let abundant_numbers: Vec<i64> = (1..28124).filter(is_abundant).collect();

    let sum_numbers_sum_of_two_abundant_numbers = abundant_numbers
        .iter()
        .cartesian_product(abundant_numbers.iter())
        .map(|(i, j)| i + j)
        .filter(|x| x <= &28123)
        .unique()
        .sum::<i64>();

    let sum_numbers_not_sum_of_two_abundant_numbers: i64 =
        (1..28124).sum::<i64>() - sum_numbers_sum_of_two_abundant_numbers;

    println!("{} == 4179871", sum_numbers_not_sum_of_two_abundant_numbers);
}
