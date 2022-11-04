// Double-base palindromes
// The decimal number, 585 = 10010010012 (binary), is palindromic in both bases.
//
// Find the sum of all numbers, less than one million, which are palindromic in base 10 and base 2.
//
// (Please note that the palindromic number, in either base, may not include leading zeros.)

fn is_palindromic_bin(i: &u32) -> bool {
    let s = format!("{:b}", i);
    return s == s.chars().rev().collect::<String>();
}

fn is_palindromic_dec(i: &u32) -> bool {
    let s = format!("{}", i);
    return s == s.chars().rev().collect::<String>();
}

fn problem_036() -> u32 {
    (1..1000000)
        .filter(is_palindromic_dec)
        .filter(is_palindromic_bin)
        .sum::<u32>()
}

fn main() {
    println!("{}", problem_036());
}

#[cfg(test)]
mod tests {
    use crate::problem_036;

    #[test]
    fn test_problem_036() {
        let result = problem_036();
        assert_eq!(result, 872187);
    }
}
