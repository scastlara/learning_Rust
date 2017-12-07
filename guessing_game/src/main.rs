extern crate rand;

use std::io;
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1,101);
    println!("Guess the number:");
    println!("input guess.");
    
    let mut guess = String::new();
    let mut done = false;

    while !done {
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
    
        // Checks if guess is indeed a number.e
        let guess_num: u32 = guess.trim().parse()
            .expect("Please type a number!"); 
        
        if guess_num < secret_number {
            println!("Too small!");
            guess = String::new();
        } else if guess_num > secret_number {
            println!("Too big!");
            guess = String::new();
        } else {
            println!("You guessed it!");
            done = true;
        }
    }
    
    println!("You guessed: {}", guess);
}
