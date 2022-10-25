use project_euler_solutions::series::fib::fibonacci;

fn brute_force(mx: u64) -> u64 {
    let mut sum = 0;
    for i in fibonacci().skip(2) {
        if i > mx {
            break;
        }
        if i % 2 == 0 {
            sum += i;
        }
    }
    return sum;
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
    let mut sum = 0;
    for i in even_fibonacci().skip(1) {
        if i > mx {
            break;
        }
        if i % 2 == 0 {
            sum += i;
        }
    }
    return sum;
}

fn main() {
    let mx = 4000000;
    println!("{} {}", brute_force(mx), three_times_faster(mx));
}
