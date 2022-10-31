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

fn main() {
    for n in 2..10 {
        recurrence_length(n, true);
    }

    let longest = (1..1000)
        .map(|n| (n, recurrence_length(n, false)))
        .max_by(|(_, r1), (_, r2)| r1.cmp(r2))
        .unwrap()
        .0;
    println!("Longest recurring cycle is 1/{}", longest);
    recurrence_length(longest, true);
}
