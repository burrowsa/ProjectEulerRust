use project_euler_solutions::factors::factors;
use project_euler_solutions::series::triangles::triangles;

fn main() {
    let n = 500;
    for i in triangles() {
        if factors(i).count() * 2 > n {
            println!("{}", i);
            break;
        }
    }
}
