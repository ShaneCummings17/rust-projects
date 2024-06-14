// 7. Determine how many of the characters are vowels and how many
// are consonants in a given line of text. Also terminate the string
// when the input character encountered is other than the alphabet.
use std::io;

fn main() {
    let string: String;
    
    loop {
        println!("Please enter an alphabetical string.");
        let user_input: String = get_user_input();
        if !user_input.chars().all(char::is_alphabetic) {
            continue
        }

        string = user_input;
        break;
    }

    const VOWELS: [char; 5] = ['a','e', 'i', 'o', 'u'];
    let mut vowel_count: u32 = 0;
    let mut consonant_count: u32 = 0;

    for c in string.chars() {
        if VOWELS.contains(&c) {
            vowel_count += 1;
        } else {
            consonant_count += 1;
        }
    }

    println!("The string contains {consonant_count} consonants and {vowel_count} vowels.");

}

fn get_user_input() -> String {
    // Get user input
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Unable to read line.");

    // Return value
    return String::from(user_input.to_lowercase().trim())

}