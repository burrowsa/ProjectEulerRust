pub fn sum_digits<T: ToString>(i: T) -> i64 {
    return i
        .to_string()
        .chars()
        .map(|x| x.to_string().parse::<i64>().unwrap())
        .sum::<i64>();
}