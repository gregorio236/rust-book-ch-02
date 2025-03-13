use rand::random_range;
use std::{cmp::Ordering, io::stdin};

fn main() {
    println!("Guess the number!");

    let secret_number = random_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        stdin().read_line(&mut guess).expect("Failed to read line!");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                print!("Please type a number!");
                continue;
            }
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
            Ordering::Greater => println!("Too big!"),
        };
    }
}
