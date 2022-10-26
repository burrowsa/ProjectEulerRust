extern crate num;
use num_bigint::ToBigUint;
use project_euler_solutions::misc::sum_digits;


fn main() {
    let n = 100;
    let mut result = n.to_biguint().unwrap();
    for i in 1..n {
        result *= i.to_biguint().unwrap();
    }

    print!("{}", sum_digits(result));
}
