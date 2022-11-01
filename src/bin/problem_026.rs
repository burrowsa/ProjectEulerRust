// Reciprocal cycles
// A unit fraction contains 1 in the numerator. The decimal representation of the unit fractions with denominators 2 to 10 are given:
//
// 1/2	= 	0.5
// 1/3	= 	0.(3)
// 1/4	= 	0.25
// 1/5	= 	0.2
// 1/6	= 	0.1(6)
// 1/7	= 	0.(142857)
// 1/8	= 	0.125
// 1/9	= 	0.(1)
// 1/10	= 	0.1
//
// Where 0.1(6) means 0.166666..., and has a 1-digit recurring cycle. It can be seen that 1/7 has a 6-digit recurring cycle.
//
// Find the value of d < 1000 for which 1/d contains the longest recurring cycle in its decimal fraction part.

fn recurrence_length(n: i64, debug: bool) -> usize {
    let mut rem = 10;
    let mut rems = vec![rem];
    let mut digits = vec![0];
    let mut start: Option<usize> = None;
    while rem > 0 && start.is_none() {
        digits.push(rem / n);
        rem = rem % n * 10;
        match rems.iter().position(|x| x == &rem) {
            Some(pos) => {
                start = Some(pos);
                break;
            }
            None => rems.push(rem),
        }
    }
    let recurrences = match start {
        Some(s) => rems.len() - s,
        None => 0,
    };
    if debug {
        print!("1/{} = ", n);
        for (i, d) in digits.iter().enumerate() {
            if let Some(j) = start {
                if j + 1 == i {
                    print!("(");
                }
            }
            print!("{}", d);
            if i == 0 {
                print!(".");
            }
        }
        if start.is_some() {
            print!(")");
        }
        println!(" [{}-digit recurring cycle]", recurrences);
    }
    return recurrences;
}

fn problem_026() -> i64 {
    return (1..1000)
        .map(|n| (n, recurrence_length(n, false)))
        .max_by(|(_, r1), (_, r2)| r1.cmp(r2))
        .unwrap()
        .0;
}

fn main() {
    let longest = problem_026();
    println!("Longest recurring cycle is 1/{}", longest);
    recurrence_length(longest, true);
}

#[cfg(test)]
mod tests {
    use crate::{problem_026, recurrence_length};

    #[test]
    fn test_recurrence_length_2() {
        assert_eq!(recurrence_length(2, false), 0)
    }

    #[test]
    fn test_recurrence_length_3() {
        assert_eq!(recurrence_length(3, false), 1)
    }

    #[test]
    fn test_recurrence_length_7() {
        assert_eq!(recurrence_length(7, false), 6)
    }

    #[test]
    fn test_problem_026() {
        assert_eq!(problem_026(), 983)
    }
}
