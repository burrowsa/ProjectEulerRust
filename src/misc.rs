pub struct Digits {
    i: u64,
}

impl Iterator for Digits {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        return if self.i != 0 {
            let ret = self.i % 10;
            self.i = self.i / 10;
            Some(ret)
        } else {
            None
        };
    }
}

pub fn digits(i: u64) -> Digits {
    Digits { i }
}

pub fn sum_digits<T: ToString>(i: T) -> i64 {
    return i
        .to_string()
        .chars()
        .map(|x| x.to_string().parse::<i64>().unwrap())
        .sum::<i64>();
}
