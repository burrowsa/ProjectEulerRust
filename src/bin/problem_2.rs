use project_euler_solutions::series::fib::fibonacci;

fn brute_force(mx: u64) -> u64 {
    return fibonacci::<u64>()
        .skip(2)
        .take_while(|i| i <= &mx)
        .filter(|i| i % 2 == 0)
        .sum();
}

struct EvenFibonacci {
    curr: u64,
    next: u64,
}

impl Iterator for EvenFibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.curr;

        self.curr = self.next;
        self.next = current + 4 * self.next;

        Some(current)
    }
}

fn even_fibonacci() -> EvenFibonacci {
    EvenFibonacci { curr: 0, next: 2 }
}

fn three_times_faster(mx: u64) -> u64 {
    return even_fibonacci()
    .take_while(|i| i <= &mx)
    .sum();
}

fn main() {
    let mx = 4000000;
    println!("{} {}", brute_force(mx), three_times_faster(mx));
}
