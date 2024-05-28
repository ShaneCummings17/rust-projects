// Write a program to find the factorial of a number
// entered by the user (check for all conditions).
use std::io;

fn main() {
    println!("Welcome to the factorial calculator!");
    let answer: u128;

    // Get user input
    loop {
        let mut user_input = String::new();
        println!("Please enter a number");
        io::stdin()
            .read_line(&mut user_input)
            .expect("Unable to read input from the terminal");

        // Check that it's a positive number input
        let user_input: u128 = match user_input.trim().parse() {
            Ok(val) => val,
            Err(_) => {
                user_input.clear();
                println!("Your input is not a positive number!");
                continue
            }
        };

        answer = match factorial(&user_input) {
            Some(num) => num,
            None => {
                println!("The factorial is too high for the machine to calculate!");
                continue
            }
        };

        break
    }
    
    // Calculate the factorial
    println!("Your factorial is: {}", answer);
}

// Not an optimized algorithm, but who really cares? lol
fn factorial(num: &u128) -> Option<u128> {
    // Return 1 if the number is 0 or 1
    if *num == 0 || *num == 1 {
        return Some(1);
    }

    // Calculate the factorial
    let mut factorial: u128 = 1;
    for i in 2..=*num {
        factorial = factorial.checked_mul(i)?;
    }

    return Some(factorial);
}