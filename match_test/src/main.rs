#[derive(Debug)]
#[allow(dead_code)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    California,
    Texas,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Yucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("Quarter from state {:?}!", state);
            match state {
                UsState::Arizona => println!("This is a special print for Arizona!"),
                // For match ops, underscore is the Rust "default" equivalent for everything else
                _ => (),
            }
            /*
             * The 'if let' operator can be used in place of a match with a single arm.  See:
             *
             * if let UsState::Arizona = state {
             *      println!("This is a special print for Arizona!");
             * } else {
             *      ()
             * }
             */
            25
        },
    }
}

fn main() {
    let c1 = Coin::Penny;
    let c2 = Coin::Nickel;
    let c3 = Coin::Dime;
    let c4 = Coin::Quarter(UsState::Arizona);

    println!("c1 is {} cents!", value_in_cents(c1));
    println!("c2 is {} cents!", value_in_cents(c2));
    println!("c3 is {} cents!", value_in_cents(c3));
    println!("c4 is {} cents!", value_in_cents(c4));
}
