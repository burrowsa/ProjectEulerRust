use itertools::Itertools;
use project_euler_solutions::factors::factors;

fn main() {
    let sum = (4000..8000)
        .filter(|n| {
            factors(*n).any(|(i, j)| {
                i.to_string()
                    .chars()
                    .chain(j.to_string().chars())
                    .chain(n.to_string().chars())
                    .sorted()
                    .collect::<String>()
                    == "123456789"
            })
        })
        .sum::<i64>();

    println!("{} = 45228", sum);
}
