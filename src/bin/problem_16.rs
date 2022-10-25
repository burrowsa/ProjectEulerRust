use num_bigint::ToBigInt;

fn main() {
    let n = 1000;
    let two = 2i64.to_bigint().unwrap();
    let two_to_n = two.pow(n);
    println!(
        "{}",
        two_to_n
            .to_string()
            .chars()
            .map(|x| x.to_string().parse::<i32>().unwrap())
            .sum::<i32>()
    );
}
