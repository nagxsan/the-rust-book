/// A simple guessing game project to get a flavour of coding in Rust

// Import the standard io package
use std::io;
use std::cmp::Ordering;

// Import the Rng trait from the rand crate
use rand::Rng;

fn main() {
    loop {
        // A simple print statement to the stdout
        println!("Guess the number! Please input your guess.");

        // Define a mutable variable of type String
        // In Rust, variables are immutable by default and to specifically change the value of a variable further on
        // declare it as mutable by using the `mut` keyword
        // `new` is an associated function: An associated function is a function that’s implemented on a type
        let mut guess = String::new();

        let num_bytes= io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("Number of bytes: {}", num_bytes);
        println!("You guessed: {}", guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        let secret_number: u32 = rand::rng().random_range(1..=10);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
            Ordering::Greater => println!("Too big!"),
        }
    }
}