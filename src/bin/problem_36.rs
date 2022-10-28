fn is_palindromic_bin(i: &i64) -> bool {
    let s = format!("{:b}", i);
    return s == s.chars().rev().collect::<String>();
}

fn is_palindromic_dec(i: &i64) -> bool {
    let s = format!("{}", i);
    return s == s.chars().rev().collect::<String>();
}

fn main() {
    println!(
        "{} == 872187",
        (1..1000000)
            .filter(is_palindromic_dec)
            .filter(is_palindromic_bin)
            .sum::<i64>()
    );
}
