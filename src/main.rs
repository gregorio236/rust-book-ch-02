use rand::random_range;
use std::io::stdin;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    stdin().read_line(&mut guess).expect("Failed to read line!");

    println!("You guessed: {guess}");

    let secret_number = random_range(1..=100);

    println!("The secret number is: {secret_number}");
}
