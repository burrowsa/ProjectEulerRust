use project_euler_solutions::numbers::int_to_words;

fn main() {
    let n = 1000;
    let mut total = 0;
    for i in 1..(n + 1) {
        let words = int_to_words(i);
        // println!("{}", words);
        total += words.replace(" ", "").replace("-", "").len();
    }
    println!("{} = 21124", total);
}
