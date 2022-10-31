fn main() {
    let n = 1000;

    for a in 1..n {
        for b in (a + 1)..n {
            let c = n - (a + b);
            if a * a + b * b == c * c {
                println!("{}² + {}² = {}²", a, b, c);
                println!("{} * {} * {} = {}", a, b, c, a * b * c);
            }
        }
    }
}
