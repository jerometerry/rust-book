extern crate rand;
extern crate rust_book;

use rust_book::phrases::english::{greetings as en_greetings, farewells as en_farewells};
use rust_book::phrases::japanese::{greetings as ja_greetings, farewells as ja_farewells};

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less		=> println!("Too small!"),
            Ordering::Greater	=> println!("Too big!"),
            Ordering::Equal		=> {
                println!("You win!");
                break;
            }
        }
    }

    print_phrases();
}

fn print_phrases() {
    println!("Hello in English: {}", en_greetings::hello());
    println!("Goodbye in English: {}", en_farewells::goodbye());
    println!("Hello in Japanese: {}", ja_greetings::hello());
    println!("Goodbye in Japanese: {}", ja_farewells::goodbye());
}

