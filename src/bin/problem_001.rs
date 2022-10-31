use itertools::Itertools;
use project_euler_solutions::series::ap::sum_n_terms_ap;
use std::collections::HashSet;

fn brute_force(n: i64, factors: &[i64]) -> i64 {
    let mut sum = 0;

    for i in 1..n {
        for j in factors {
            if i % j == 0 {
                sum += i;
                break;
            }
        }
    }
    return sum;
}

fn sum_multiples_of_i_less_than_n(n: i64, i: i64) -> i64 {
    return sum_n_terms_ap((n - 1) / i + 1, 0, i);
}

fn remove_multiples(numbers: &[i64]) -> Vec<i64> {
    let mut out = Vec::new();
    let mut sorted = numbers.to_vec();
    sorted.sort();
    for i in sorted {
        let mut add = true;
        for j in &out {
            add = add && (i % *j) != 0;
        }
        if add {
            out.push(i);
        }
    }
    return out;
}

fn combined_factors(numbers: &[i64]) -> HashSet<i64> {
    let mut out = HashSet::new();

    let simplified = remove_multiples(numbers);

    for i in 1..=simplified.len() {
        for c in simplified.iter().combinations(i) {
            let mut product = -1;
            for j in c {
                product *= -j;
            }
            out.insert(product);
        }
    }

    return out;
}

fn algebraic_solution(n: i64, factors: &[i64]) -> i64 {
    let mut sum = 0;
    for i in combined_factors(&factors) {
        sum += i.signum() * sum_multiples_of_i_less_than_n(n, i.abs());
    }
    return sum;
}

fn main() {
    let n = 1000;
    let factors = [3, 5];
    println!(
        "{} {}",
        brute_force(n, &factors),
        algebraic_solution(n, &factors)
    );
}
