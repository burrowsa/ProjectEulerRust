static COINS: [usize; 8] = [1, 2, 5, 10, 20, 50, 100, 200];
static mut WAYS: [u32; 201] = [0; 201];
static mut READY: bool = false;

fn number_of_ways_to_make(n: usize) -> u32 {
    unsafe {
        if !READY {
            WAYS[0] = 1;
            for coin in COINS {
                for i in coin..=200 {
                    WAYS[i] += WAYS[i - coin];
                }
            }
            READY = true;
        }
        return WAYS[n];
    }
}

fn main() {
    println!("{} == 1", number_of_ways_to_make(0));
    println!("{} == 1 (1)", number_of_ways_to_make(1));
    println!("{} == 2 (2 11)", number_of_ways_to_make(2));
    println!("{} == 4 (5 221 2111 11111)", number_of_ways_to_make(5));
    println!("{} == 73682", number_of_ways_to_make(200));
}
