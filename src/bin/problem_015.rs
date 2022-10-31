use memoize::memoize;

#[memoize]
fn count_paths(state: (i64, i64)) -> i64 {
    let (r, d) = state;
    if r == 0 || d == 0 {
        return 1;
    } else {
        return if r > 0 { count_paths((r - 1, d)) } else { 0 }
            + if d > 0 { count_paths((r, d - 1)) } else { 0 };
    }
}

fn main() {
    let side = 20;
    println!("{} = 137846528820", count_paths((side, side)));
    // Analytical answer possible from using https://en.wikipedia.org/wiki/Binomial_coefficient
}
