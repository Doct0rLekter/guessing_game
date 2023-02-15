// Make clippy pedantic
#![warn(clippy::pedantic)]

use std::io::{stdin, stdout, Write};

fn main() {
    println!("Guess the number!");

    print!("Please input your guess: ");
    // flush buffer
    stdout().flush().unwrap();

    let mut guess = String::new();

    stdin().read_line(&mut guess).expect("Failed to read line");

    println!("you guessed: {guess}");
}
