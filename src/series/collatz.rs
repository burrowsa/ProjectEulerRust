pub struct Collatz {
    next: usize,
    stop: bool,
}

impl Iterator for Collatz {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.stop {
            None
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
            Some(r)
        }
    }
}

pub fn collatz(start: usize) -> Collatz {
    Collatz {
        next: start,
        stop: false,
    }
}

#[cfg(test)]
mod tests {
    use super::collatz;

    #[test]
    fn collatz_1() {
        let items: Vec<usize> = collatz(1).take(1000).collect();
        assert_eq!(items, vec![1])
    }

    #[test]
    fn collatz_2() {
        let items: Vec<usize> = collatz(2).take(1000).collect();
        assert_eq!(items, vec![2, 1])
    }

    #[test]
    fn collatz_3() {
        let items: Vec<usize> = collatz(3).take(1000).collect();
        assert_eq!(items, vec![3, 10, 5, 16, 8, 4, 2, 1])
    }

    #[test]
    fn collatz_4() {
        let items: Vec<usize> = collatz(4).take(1000).collect();
        assert_eq!(items, vec![4, 2, 1])
    }

    #[test]
    fn collatz_5() {
        let items: Vec<usize> = collatz(5).take(1000).collect();
        assert_eq!(items, vec![5, 16, 8, 4, 2, 1])
    }

    #[test]
    fn collatz_10() {
        let items: Vec<usize> = collatz(10).take(1000).collect();
        assert_eq!(items, vec![10, 5, 16, 8, 4, 2, 1])
    }

    #[test]
    fn collatz_1280() {
        let items: Vec<usize> = collatz(1280).take(1000).collect();
        assert_eq!(
            items,
            vec![1280, 640, 320, 160, 80, 40, 20, 10, 5, 16, 8, 4, 2, 1]
        )
    }

    #[test]
    fn collatz_1281() {
        let items: Vec<usize> = collatz(1281).take(1000).collect();
        assert_eq!(
            items,
            vec![
                1281, 3844, 1922, 961, 2884, 1442, 721, 2164, 1082, 541, 1624, 812, 406, 203, 610,
                305, 916, 458, 229, 688, 344, 172, 86, 43, 130, 65, 196, 98, 49, 148, 74, 37, 112,
                56, 28, 14, 7, 22, 11, 34, 17, 52, 26, 13, 40, 20, 10, 5, 16, 8, 4, 2, 1
            ]
        )
    }
}
