use rand::prelude::*;
use std::thread;
use std::time::Duration;
use std::collections::HashMap;

fn main() {
    let simulated_user_specified_value: u32 = thread_rng().gen_range(0, 100);
    let simulated_random_number: u32 = thread_rng().gen_range(0, 100);

    println!(
        "val {} random {}",
        simulated_user_specified_value, simulated_random_number
    );

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

struct Catcher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: HashMap<u32, Option<u32>>,
}

impl<T> Catcher<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Catcher<T> {
        Catcher {
            calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value.get(&arg) {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value.insert(v, Some(v));
                v
            },
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Catcher::new(|num| {
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 50 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number % 3 == 0 {
            println!("Take a break today and stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_result.value(intensity));
        }

    }
}
