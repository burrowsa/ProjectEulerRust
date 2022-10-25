use project_euler_solutions::factors::factors;
use std::cmp::max;
use std::collections::HashMap;

fn _totup_primes(i: i64, primes: &mut HashMap<i64, i64>) {
    let fs: Vec<(i64, i64)> = factors(i).collect();
    if fs.len() == 1 {
        let p = primes.entry(i).or_insert(0);
        *p += 1;
    } else {
        let (f1, f2) = fs[fs.len() - 1];
        _totup_primes(f1, primes);
        _totup_primes(f2, primes);
    }
}

fn totup_primes(i: i64) -> HashMap<i64, i64> {
    let mut result = HashMap::new();
    _totup_primes(i, &mut result);
    return result;
}

fn merge_primes(p1: &mut HashMap<i64, i64>, p2: HashMap<i64, i64>) {
    for i in p2.keys() {
        let p = p1.entry(*i).or_insert(0);
        *p = max(*p, *p2.get(i).unwrap());
    }
}

fn simplify(n: i64) -> Vec<i64> {
    let mut primes = HashMap::new();

    for i in 1..(n + 1) {
        let i_primes = totup_primes(i);
        merge_primes(&mut primes, i_primes);
    }

    let mut result = Vec::new();
    for i in primes.keys() {
        for _ in 0..(*primes.get(i).unwrap()) {
            result.push(*i);
        }
    }
    return result;
}

fn main() {
    let simple = simplify(20);
    let result: i64 = simple.iter().product();
    println!("{}", result);
}
