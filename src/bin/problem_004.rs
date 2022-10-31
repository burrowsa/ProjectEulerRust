fn is_palindrome(i: i64) -> bool {
    let s = i.to_string();
    let r = s.chars().rev().collect::<String>();
    return s == r;
}

fn main() {
    for i in (900..999).rev() {
        for j in (900..i).rev() {
            if is_palindrome(i * j) {
                println!("{} = {} x {}", i * j, i, j);
                return;
            }
        }
    }
}
