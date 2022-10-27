fn main() {
    let n = 5;
    println!(
        "{} == 443839",
        (2..999999u64)
            .filter(|i| *i
                == ((i % 10).pow(n)
                    + ((i / 10) % 10).pow(n)
                    + ((i / 100) % 10).pow(n)
                    + ((i / 1000) % 10).pow(n)
                    + ((i / 10000) % 10).pow(n)
                    + ((i / 100000) % 10).pow(n)
                    + ((i / 1000000) % 10).pow(n)))
            .sum::<u64>()
    );
}
