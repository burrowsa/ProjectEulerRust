use num::BigUint;
use num_bigint::ToBigUint;
use project_euler_solutions::series::fib::fibonacci;

fn main() {
    let n: u32 = 1000;
    let digits = 10.to_biguint().unwrap().pow(n-1);
    let x = fibonacci::<BigUint>()
        .enumerate()
        .find(|(_, x)| *x >= digits)
        .unwrap()
        .0;
    println!("{} == 4782", x);
}
