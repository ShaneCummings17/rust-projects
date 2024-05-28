// Write a currency converter from Euro to Dollar and vice versa.
use dialoguer::Select;
use std::io;


fn main() {
    println!("Welcome to the currency converter!");

    const CURRENCIES: [&str; 2] = ["Euro", "Dollar"];
    const EXCHANGE_RATE: f32 = 1.09; // How many dollars are in a Euro

    // Select the starting currency
    let currency_index = Select::new()
        .with_prompt("Please select your starting currency")
        .items(&CURRENCIES)
        .interact()
        .unwrap();
    
    let currency = CURRENCIES[currency_index];
    println!("Selected currency: {currency}");
    
    // Input the amount of money to exchange
    println!("Please input the amount of money you wish to exchange:");

    let clean_number: f32;
    // Ensure a number was entered
    loop {
        let mut amount = String::new();
        io::stdin()
            .read_line(&mut amount)
            .expect("Unable to read line.");

        let amount: f32 = match amount.trim().parse() {
            Ok(val) => val,
            Err(_) => {
                amount.clear();
                println!("You did not input a positive amount of money!");
                continue
            }
        };
        clean_number = amount;
        break;
    }

    let exchanged_amount: f32;
    let exchanged_currency: &str;
    if currency == "Euro" {
        exchanged_amount = clean_number * EXCHANGE_RATE;
        exchanged_currency = "Dollar";
    } else {
        exchanged_amount = clean_number / EXCHANGE_RATE;
        exchanged_currency = "Euro";
    }

    println!("Your exchanged value is {:.2} {exchanged_currency}s", exchanged_amount);
}