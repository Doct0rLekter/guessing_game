// Make clippy pedantic
#![warn(clippy::pedantic)]

use rand::Rng;
use std::cmp::Ordering;
use std::io::{stdin, stdout, Write};

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");
    let mut tries = 0;
    
    loop {
        tries += 1;
        if game_loop(secret_number) {
            println!("You took {tries} tries to guess the number");
            break;
        }
    }
}

fn game_loop(the_number: u32) -> bool {
    let mut guess = String::new();

    print!("Please input your guess: ");
    // flush buffer
    stdout().flush().unwrap();

    stdin().read_line(&mut guess).expect("Failed to read line");
    let guess: u32 = guess.trim().parse().expect("Please type a positive integer!");

    println!("you guessed: {guess}");

    match guess.cmp(&the_number) {
        Ordering::Less => {
            println!("Too small!");
            false
        }
        Ordering::Greater => {
            println!("Too big!");
            false
        }
        Ordering::Equal => {
            println!("You win!");
            true
        }
    }
}
