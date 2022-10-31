use project_euler_solutions::factors::is_prime;

fn nth_prime(n: i64) -> i64 {
    let mut n_primes = 0;
    let mut i = 1;
    while n_primes < n {
        i += if i > 2 { 2 } else { 1 };
        if is_prime(i) {
            n_primes += 1;
        }
    }
    return i;
}

fn main() {
    println!("{}", nth_prime(10001))
}
