use project_euler_solutions::factors::sum_proper_divisors;

fn is_amicable(n: &i64) -> bool {
    let sum_proper_divisors_n = sum_proper_divisors(*n);
    return sum_proper_divisors_n != *n && sum_proper_divisors(sum_proper_divisors_n) == *n;
}

fn main() {
    println!("{}", (1..10000).filter(is_amicable).sum::<i64>());
}
