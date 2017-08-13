use std::io;

fn main () {
    println!("Guess the number");
    println!("Your guess?");
    
    let mut guess = String::new();
    
    io::stdin().read_line(&mut guess)
               .expect("Failed to read line");
    
    println!("Your guess: {}", guess);
}

