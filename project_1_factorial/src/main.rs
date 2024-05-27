// Write a program to find the factorial of a number
// entered by the user (check for all conditions).
use std::io;

fn main() {
    println!("Welcome to the factorial calculator!");

    // Get user input
    let clean_number;

    loop {
        let mut user_input = String::new();
        println!("Please enter a number");
        io::stdin()
            .read_line(&mut user_input)
            .expect("Unable to read input from the terminal");

        // Check that it's a positive number input
        let user_input: u32 = match user_input.trim().parse() {
            Ok(val) => val,
            Err(_) => {
                user_input.clear();
                println!("Your input is not a positive number!");
                continue
            }
        };

        clean_number = user_input;
        break
    }
    
    println!("User selected {}", clean_number);

    // Calculate the factorial
}