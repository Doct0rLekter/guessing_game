// Make clippy pedantic
#![warn(clippy::pedantic)]

use rand::Rng;
use std::cmp::Ordering;
use std::io::{stdin, stdout, Write};

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");

    print!("Please input your guess: ");
    // flush buffer
    stdout().flush().unwrap();

    let mut guess = String::new();

    stdin().read_line(&mut guess).expect("Failed to read line");
    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("you guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
