struct Spiral {
    n: u32,
    diagonals: u32,
    step: u32,
    size: u32,
}

impl Iterator for Spiral {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.n;
        if self.diagonals == 4 {
            self.diagonals = 0;
            self.step += 2;
        }
        self.n += self.step;
        self.diagonals += 1;
        return if n <= self.size * self.size {
            Some(n)
        } else {
            None
        };
    }
}

fn main() {
    let result: u32 = Spiral {
        n: 1,
        size: 1001,
        diagonals: 0,
        step: 2,
    }
    .sum();
    println!("{} == 669171001", result);
}
