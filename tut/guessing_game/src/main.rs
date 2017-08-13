extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main () {
    println!("Guess the number");
    
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is {}", secret_number);
    
    println!("Your guess?");
    
    let mut guess = String::new();
    
    io::stdin().read_line(&mut guess)
               .expect("Failed to read line");
    println!("Your guess: {}", guess);
    
    let guess: u32 = guess.trim().parse().expect("That wasn't very numbery");
    println!("Your guess: {}", guess);
    
    match guess.cmp(&secret_number) {
        Ordering::Less    => println!("Too small"),
        Ordering::Greater => println!("Too big"),
        Ordering::Equal   => println!("Too equal"),
    }
}
