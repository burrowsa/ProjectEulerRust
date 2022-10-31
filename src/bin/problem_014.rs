use project_euler_solutions::series::collatz::collatz;

fn main() {
    let mut mx_start = 0;
    let mut mx = 0;
    for start in 1..1000000 {
        let chain_len = collatz(start).count();
        if chain_len > mx {
            mx = chain_len;
            mx_start = start
        }
    }
    println!("{} {}", mx_start, mx);
}
