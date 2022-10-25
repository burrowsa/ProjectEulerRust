pub struct Collatz {
    next: i64,
    stop: bool,
}

impl Iterator for Collatz {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.stop {
            return None;
        } else {
            let r = self.next;
            if r == 1 {
                self.stop = true;
            } else {
                self.next = if self.next % 2 == 0 {
                    self.next / 2
                } else {
                    3 * self.next + 1
                };
            }
            return Some(r);
        }
    }
}

pub fn collatz(start: i64) -> Collatz {
    Collatz {
        next: start,
        stop: false,
    }
}
