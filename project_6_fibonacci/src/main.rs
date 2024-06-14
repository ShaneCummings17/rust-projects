// 6. Create a program which generates Fibonacci series till a number
// 'n' where 'n' is entered by the user. For eg. if the user enters 10
// then the output would be: 1 1 2 3 5 8.

use std::io;

fn main() {
    println!("Hello, world!");
    // Get user input
    let user_input = get_user_input();


    // Print out fibonacci numbers
    println!("1");
    let mut n_minus_1: u128 = 1;
    let mut n_minus_2: u128 = 0;

    // Don't like using "_i" here; think of better way
    for _i in 2..=user_input {
        let checked_fibo_num = (n_minus_1).checked_add(n_minus_2);
        match checked_fibo_num {
            Some(num) => {
                println!("{}", num);
                n_minus_2 = n_minus_1;
                n_minus_1 = num;
            },
            None => {
                println!("Cannot compute higher than this!");
                break;
            }
        };
    }
}

fn get_user_input() -> u128 {
    // Get user input
    let number;
    loop {
        println!("Please enter a positive integer.");
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Unable to read line.");

        let user_input: u128 = match user_input.trim().parse() {
            Ok(val) => val,
            Err(_) => {
                user_input.clear();
                continue
            }
        };
        number = user_input;
        break;
    }

    // Return value
    return number

}