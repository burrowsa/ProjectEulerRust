use itertools::Itertools;
use num_bigint::ToBigUint;

fn main() {
    println!(
        "{}",
        (2..=100)
            .cartesian_product(2..=100)
            .map(|(a, b): (u32, u32)| a.to_biguint().unwrap().pow(b))
            .sorted()
            .unique()
            .count(),
    )
}
