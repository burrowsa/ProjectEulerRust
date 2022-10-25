use project_euler_solutions::factors::{factors, is_prime};

fn largest_prime_factor(n: i64) -> i64 {
    return factors(n)
        .map(|(i, _)| i)
        .filter(|i| is_prime(*i))
        .max().unwrap();
}

fn main() {
    let n = 600851475143;
    println!("{} == 6857", largest_prime_factor(n));
}
