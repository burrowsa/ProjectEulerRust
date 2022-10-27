use itertools::Itertools;
use num::ToPrimitive;
use project_euler_solutions::factors::is_prime;
use std::i64;

struct Remarkable {
    a: i64,
    b: i64,
    n: usize,
}

impl Iterator for Remarkable {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.n.to_i64().unwrap();
        self.n += 1;
        let result = n * n + self.a * n + self.b;
        return if is_prime(result) { Some(result) } else { None };
    }
}

fn num_remarkable_primes(a: i64, b: i64) -> usize {
    return Remarkable { a, b, n: 0 }.count();
}

fn main() {
    println!("a=1 b=41 gives {} primes", num_remarkable_primes(1, 41));
    println!(
        "a=-79 b=1601 gives {} primes",
        num_remarkable_primes(-79, 1601)
    );

    let ((a, b), primes) = (-999..1000)
        .cartesian_product(-1000..1001)
        .map(|(a, b)| ((a, b), num_remarkable_primes(a, b)))
        .max_by(|(_, primes1), (_, primes2)| primes1.cmp(primes2))
        .unwrap();

    println!("a={} b ={} gives {} primes", a, b, primes);
    println!("{} == -59231", a * b);
}
