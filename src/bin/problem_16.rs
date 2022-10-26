use num_bigint::ToBigInt;
use project_euler_solutions::misc::sum_digits;

fn main() {
    let n = 1000;
    let two = 2i64.to_bigint().unwrap();
    let two_to_n = two.pow(n);
    println!(
        "{}",
        sum_digits(two_to_n)
    );
}
