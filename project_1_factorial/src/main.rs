// Write a program to find the factorial of a number
// entered by the user (check for all conditions).
use std::io;

fn main() {
    println!("Welcome to the factorial calculator! Please enter a number:");

    // Get user input
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Unable to read input from the terminal");

    println!("User input is {}", user_input.trim());

    // Check that it's valid input

    // Calculate the factorial
}
