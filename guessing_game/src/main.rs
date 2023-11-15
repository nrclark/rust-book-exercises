#![warn(clippy::pedantic)]

use rand::Rng;
use std::cmp::Ordering;
use std::env;
use std::io;
use std::io::Write;

fn main() {
    let mut wizard: bool = false;

    if env::var("WIZARD").is_ok() {
        wizard = true;
    }

    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    if wizard {
        println!("The secret number is {secret_number}");
    }

    loop {
        print!("Please input your guess: ");
        io::stdout().flush().unwrap();
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(n) => n,
            Err(e) => {
                println!("Error: {e}");
                continue;
            }
        };

        if wizard {
            println!("You guessed: {guess}");
        }

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
