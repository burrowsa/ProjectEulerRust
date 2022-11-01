use itertools::Itertools;
use num::ToPrimitive;

// Digit cancelling fractions
// The fraction 49/98 is a curious fraction, as an inexperienced mathematician in attempting to simplify it may incorrectly believe that
// 49/98 = 4/8, which is correct, is obtained by cancelling the 9s.
//
// We shall consider fractions like, 30/50 = 3/5, to be trivial examples.
//
// There are exactly four non-trivial examples of this type of fraction, less than one in value, and containing two digits in the
//  numerator and denominator.
//
// If the product of these four fractions is given in its lowest common terms, find the value of the denominator.

fn is_curious_frac((n, d): &(usize, usize)) -> bool {
    if n >= d || (n % 10 == 0 && d % 10 == 0) {
        return false;
    }

    let frac = n.to_f64().unwrap() / d.to_f64().unwrap();
    return (n % 10 == d % 10
        && (frac - (n / 10).to_f64().unwrap() / (d / 10).to_f64().unwrap()).abs() <= f64::EPSILON)
        || (n % 10 == d / 10
            && (frac - (n / 10).to_f64().unwrap() / (d % 10).to_f64().unwrap()).abs()
                <= f64::EPSILON)
        || (n / 10 == d % 10
            && (frac - (n % 10).to_f64().unwrap() / (d / 10).to_f64().unwrap()).abs()
                <= f64::EPSILON)
        || (n / 10 == d / 10
            && (frac - (n % 10).to_f64().unwrap() / (d % 10).to_f64().unwrap()).abs()
                <= f64::EPSILON);
}

fn problem_033() -> (usize, usize) {
    return (10..100)
        .cartesian_product(10..100)
        .filter(is_curious_frac)
        .fold((1, 1), |(n1, d1), (n2, d2)| (n1 * n2, d1 * d2));
}

fn main() {
    let result = problem_033();
    println!("{}", result.1 / result.0);
}

#[cfg(test)]
mod tests {
    use crate::problem_033;

    #[test]
    fn test_problem_033() {
        let result = problem_033();
        assert_eq!(result.1 / result.0, 100);
    }
}
