use num::ToPrimitive;
use project_euler_solutions::factors::is_prime;

struct RotDigits {
    next: Option<u64>,
    first: u64,
    pow_ten: u64,
}

impl Iterator for RotDigits {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.next;
        match ret {
            None => (),
            Some(cur) => {
                let next = (cur % 10) * self.pow_ten + cur / 10;
                self.next = if next == self.first {
                    None
                } else {
                    Some(next)
                };
            }
        };
        return ret;
    }
}

fn rotate_digits(i: u64) -> RotDigits {
    RotDigits {
        first: i,
        next: Some(i),
        // Can't do log of integer (https://github.com/rust-lang/rust/issues/70887) so get the len of the string repr
        pow_ten: 10u64.pow(i.to_string().len().to_u32().unwrap() - 1),
    }
}

fn is_circular_prime(i: &u64) -> bool {
    return rotate_digits(*i).all(|x| is_prime(x.to_i64().unwrap()));
}

pub fn main() {
    println!("{} == 13", (1..100).filter(is_circular_prime).count());
    println!("{} == 55", (1..1000000).filter(is_circular_prime).count());
}
