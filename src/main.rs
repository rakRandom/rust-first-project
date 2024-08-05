// Imports
use std::{
    cmp::Ordering, 
    io::{self, Write}
};
use rand::Rng;


// Main function
fn main() {
    println!("Guess the number between 1 and 100 (inclusive)!");

    // Generating a random number between 1 and 10 (inclusive)
    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

    // Main loop
    loop {
        print!("Please input your guess: ");
        let _ = io::stdout().flush();  // This code is used to print the String above immediately

        // Getting user guess
        let mut guess: String = String::new();  // Buffer to recieve the input

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");  // Passing what was written from the terminal to the buffer

        let guess: u32 = 
        match guess.trim().parse() { 
            Ok(num) => num,
            Err(_) => { 
                println!("Input a \x1b[4mNumber\x1b[0m between 1 and 100."); 
                continue; 
            }
        }; // Parsing the buffer from String to u(nsigned)32
        

        // Showing the result
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too low."),
            Ordering::Greater => println!("Too high."),
            Ordering::Equal => {
                println!("You got it!");
                break;
            }
        }
    }
}
