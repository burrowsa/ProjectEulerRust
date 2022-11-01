use std::iter;

// Coin sums
// In the United Kingdom the currency is made up of pound (£) and pence (p). There are eight coins in general circulation:
//
// 1p, 2p, 5p, 10p, 20p, 50p, £1 (100p), and £2 (200p).
// It is possible to make £2 in the following way:
//
// 1×£1 + 1×50p + 2×20p + 1×5p + 1×2p + 3×1p
// How many different ways can £2 be made using any number of coins?

static COINS: [usize; 8] = [1, 2, 5, 10, 20, 50, 100, 200];

fn number_of_ways_to_make(n: usize) -> u32 {
    let mut ways: Vec<u32> = iter::once(1).chain(iter::repeat(0).take(n)).collect();

    for coin in COINS {
        for i in coin..=n {
            ways[i] += ways[i - coin];
        }
    }
    return ways[n];
}

fn main() {
    println!("{}", number_of_ways_to_make(200));
}

#[cfg(test)]
mod tests {
    use crate::number_of_ways_to_make;

    #[test]
    fn test_number_of_ways_to_make_0() {
        assert_eq!(number_of_ways_to_make(0), 1);
    }

    #[test]
    fn test_number_of_ways_to_make_1() {
        assert_eq!(number_of_ways_to_make(1), 1); // 1
    }

    #[test]
    fn test_number_of_ways_to_make_2() {
        assert_eq!(number_of_ways_to_make(2), 2); // 2 11
    }

    #[test]
    fn test_number_of_ways_to_make_5() {
        assert_eq!(number_of_ways_to_make(5), 4); // 5 221 2111 11111
    }

    #[test]
    fn test_number_of_ways_to_make_6() {
        assert_eq!(number_of_ways_to_make(6), 5); // 51 222 2211 21111 111111 
    }

    #[test]
    fn test_number_of_ways_to_make_7() {
        assert_eq!(number_of_ways_to_make(7), 6); // 511 52 2221 22111 211111 1111111 
    }

    #[test]
    fn test_number_of_ways_to_make_8() {
        assert_eq!(number_of_ways_to_make(8), 7); // 5111 521 2222 22211 221111 2111111 11111111 
    }

    #[test]
    fn test_number_of_ways_to_make_200() {
        assert_eq!(number_of_ways_to_make(200), 73682);
    }
}
