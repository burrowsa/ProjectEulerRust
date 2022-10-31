use project_euler_solutions::series::squares::sum_first_n_squares;
use project_euler_solutions::series::ap::sum_n_terms_ap;

fn brute_force(n: i64) {
    let mut sum_nums = 0;
    let mut sum_sqs = 0;
    for i in 1..=n {
        sum_nums += i;
        sum_sqs += i * i;
    }
    let sum_nums_sq = sum_nums * sum_nums;
    println!("{} - {} = {}", sum_nums_sq, sum_sqs, sum_nums_sq - sum_sqs);
}

fn algebraic_solution(n: i64) {
    let sum_nums = sum_n_terms_ap(n, 1, 1);
    let sum_nums_sq = sum_nums * sum_nums;
    let sum_sqs = sum_first_n_squares(n);
    println!("{} - {} = {}", sum_nums_sq, sum_sqs, sum_nums_sq - sum_sqs);
}

fn main() {
    let n = 100;

    brute_force(n);
    algebraic_solution(n);
}
