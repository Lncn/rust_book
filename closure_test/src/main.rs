use std::thread;
use std::time::Duration;
use rand::prelude::*;

fn simulated_expensive_function(intensity: u32) -> u32 {
    println!("Calculating...");
    thread::sleep(Duration::from_secs(2));
    intensity
}


fn main() {
    let simulated_user_specified_value = simulated_expensive_function(10);
    let simulated_random_number: i32 = thread_rng().gen_range(0, 100);

    println!("val {} random {}", simulated_user_specified_value, simulated_random_number);
}
