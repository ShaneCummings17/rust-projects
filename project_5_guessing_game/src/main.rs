// 5. Guessing game. Write a game that asks the user to guess
// a number between 1 and a 100. If you guessed correctly,
//it will say you win. If your too high or too low it will also let you know.
use rand::Rng;
use std::io;

fn main () {
    // create a random number
    let random_number = rand::thread_rng().gen_range(1..=100);

    // get user input + provide feedback
    loop {
        let user_input: Option<u32> = get_user_input();
        let number: u32 = match user_input {
            None => continue,
            Some(i) => i
        };
        
        if number == random_number {
            println!("You win!");
            break;
        } else if number > 100 {
            println!("Your number is greater than 100!");
        } else if number > random_number {
            println!("Too high!");
        } else {
            println!("Too low!");
        }
    }
}

fn get_user_input() -> Option<u32> {
    // Get user input
    let mut number = String::new();
    println!("Please enter a number between 1 and 100");
    io::stdin().read_line(&mut number).expect("Unable to read the line");

    // Convert to an integer
    let number: Option<u32> = match number.trim().parse::<u32>() {
        Ok(val)  => Some(val),
        Err(_) => {
            println!("Please enter a positive integer!");
            None
        }
    };

    // Return value
    return number
    

}