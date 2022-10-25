use project_euler_solutions::factors::is_prime;

fn sum_primes_below_n(n: i64) -> i64 {
    let mut sum = 0;
    let mut i = 0;
    while i < n {
        if is_prime(i) {
            sum += i;
        }
        i += if i > 2 { 2 } else { 1 };
    }
    return sum;
}

fn main() {
    println!("{}", sum_primes_below_n(2000000))
}
