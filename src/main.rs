use rand::random_range;
use std::{cmp::Ordering, io::stdin};

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    stdin().read_line(&mut guess).expect("Failed to read line!");

    let guess: i32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed: {guess}");

    let secret_number = random_range(1..=100);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Equal => println!("You win!"),
        Ordering::Greater => println!("Too big!"),
    };
}
