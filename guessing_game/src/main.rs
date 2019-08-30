use std::io;
use std::cmp::Ordering;
use rand::prelude::*;

fn main() {
    println!("Guess the number, Lincoln!");
    let answer: u8 = random();

    loop {
        let mut guess = String::new();
        println!("Please enter your guess.");
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input!");
                continue;
            }
        };

        match guess.cmp(&answer) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => break,
        };
    }

    println!("You win!");
}
