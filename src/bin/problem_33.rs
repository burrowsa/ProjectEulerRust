use itertools::Itertools;
use num::ToPrimitive;

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

fn main() {
    let result = (10..100)
        .cartesian_product(10..100)
        .filter(is_curious_frac)
        .fold((1, 1), |(n1, d1), (n2, d2)| (n1 * n2, d1 * d2));

    println!("{} = 100", result.1 / result.0);
}
