#![deny(warnings)]

extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number");
    // Random number in the range [1; 100[, in other words 1 to 99
    let secret = rand::thread_rng().gen_range(1, 100);
    println!("The secret number is: {}", secret);

    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse()
        .expect("Not a number");

    match secret.cmp(&guess) {
        Ordering::Less => println!("It's less"),
        Ordering::Greater => println!("It's more"),
        Ordering::Equal => println!("You won !"),
    }
}