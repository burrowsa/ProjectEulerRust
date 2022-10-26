use num::{Zero, One};
use num_traits::{NumAssign};

pub struct Fibonacci<T> {
    curr: T,
    next: T,
}

impl<T:NumAssign + Clone> Iterator for Fibonacci<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.curr.clone();

        self.curr = self.next.clone();
        self.next += current.clone();

        Some(current)
    }
}

pub fn fibonacci<T: Zero + One>() -> Fibonacci<T> {
    Fibonacci {
        curr: T::zero(),
        next: T::one(),
    }
}
