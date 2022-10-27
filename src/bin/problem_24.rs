use itertools::Itertools;

fn main() {
    let millionth_perm = (0..10).permutations(10).skip(1000000-1).next().unwrap();
    for j in millionth_perm {
        print!("{}", j);
    }
    println!(" == 2783915460");
}
