use project_euler_solutions::misc::digits;

fn sum_fac_digits(n: u64) -> u64 {
    return digits(n).map(|i| (1..i + 1).product::<u64>()).sum();
}

fn main() {
    let sum: u64 = (3..1000000).filter(|n| *n == sum_fac_digits(*n)).sum();

    println!("{} == 145", sum_fac_digits(145));
    println!("{} == 40730", sum);
}
