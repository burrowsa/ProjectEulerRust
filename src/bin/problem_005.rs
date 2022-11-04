use project_euler_solutions::factors::factors;
use std::cmp::max;
use std::collections::HashMap;

// Smallest multiple
// 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
// What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?

fn _totup_primes(i: &u64, primes: &mut HashMap<u64, u64>) {
    let fs: Vec<(u64, u64)> = factors(i).collect();
    if fs.len() == 1 {
        let p = primes.entry(*i).or_insert(0);
        *p += 1;
    } else {
        let (f1, f2) = fs[fs.len() - 1];
        _totup_primes(&f1, primes);
        _totup_primes(&f2, primes);
    }
}

fn totup_primes(i: &u64) -> HashMap<u64, u64> {
    let mut result = HashMap::new();
    _totup_primes(i, &mut result);
    result
}

fn merge_primes(p1: &mut HashMap<u64, u64>, p2: HashMap<u64, u64>) {
    for (k, v2) in p2 {
        let v1 = p1.entry(k).or_insert(0);
        *v1 = max(*v1, v2);
    }
}

fn simplify(n: &u64) -> Vec<u64> {
    let mut primes = HashMap::new();

    for i in 1..=*n {
        merge_primes(&mut primes, totup_primes(&i));
    }

    let mut result = Vec::new();
    for (k, v) in primes {
        for _ in 0..v {
            result.push(k);
        }
    }
    result
}

#[cfg(test)]
fn problem_005_example() -> u64 {
    let simple = simplify(&10);
    simple.iter().product()
}

fn problem_005() -> u64 {
    let simple = simplify(&20);
    simple.iter().product()
}

fn main() {
    println!("{}", problem_005());
}

#[cfg(test)]
mod tests {
    use crate::{problem_005, problem_005_example};

    #[test]
    fn test_problem_005_example() {
        let result = problem_005_example();
        assert_eq!(result, 2520);
    }

    #[test]
    fn test_problem_005() {
        let result = problem_005();
        assert_eq!(result, 232792560);
    }
}
