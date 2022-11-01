// Special Pythagorean triplet
// A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,
//
// a² + b² = c²
//
// For example, 3² + 4² = 9 + 16 = 25 = 5².
//
// There exists exactly one Pythagorean triplet for which a + b + c = 1000.
// Find the product abc.

fn triplet_with_sum(n: i64) -> Option<(i64, i64, i64)> {
    for a in 1..n {
        for b in (a + 1)..n {
            let c = n - (a + b);
            if a * a + b * b == c * c {
                return Some((a, b, c));
            }
        }
    }
    None
}

#[cfg(test)]
fn problem_009_example() -> i64 {
    let (a, b, c) = triplet_with_sum(12).unwrap();
    a * b * c
}

#[cfg(test)]
fn problem_009() -> i64 {
    let (a, b, c) = triplet_with_sum(1000).unwrap();
    a * b * c
}

fn main() {
    let (a, b, c) = triplet_with_sum(1000).unwrap();
    println!("{}² + {}² = {}²", a, b, c);
    println!("{} * {} * {} = {}", a, b, c, a * b * c);
}

#[cfg(test)]
mod tests {
    use crate::{problem_009, problem_009_example};

    #[test]
    fn test_problem_009_example() {
        let result = problem_009_example();
        assert_eq!(result, 60);
    }

    #[test]
    fn test_problem_009() {
        let result = problem_009();
        assert_eq!(result, 31875000);
    }
}
