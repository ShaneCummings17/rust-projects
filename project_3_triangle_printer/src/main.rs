// 3. Write a program that prints out a triangle from
// smallest to largest; user inputs bottom number.
use std::io;

fn main() {
    // Get length of the triangle
    let clean_triangle_length: u32;
    // Ensure a number was entered
    loop {
        println!("Please enter a length for the triangle.");
        let mut length = String::new();
        io::stdin()
            .read_line(&mut length)
            .expect("Unable to read line.");

        let length: u32 = match length.trim().parse() {
            Ok(val) => val,
            Err(_) => {
                length.clear();
                println!("You did not input a positive length for the triangle!");
                continue
            }
        };
        clean_triangle_length = length;
        break;
    }

    print_triangle(&clean_triangle_length);
}

fn print_triangle(length: &u32) {

    println!("\nTriangle of length {}", length);
    println!("----------------------");

    for i in 0..*length+1 {
        let i_us = usize::try_from(i).unwrap();
        println!("{:*<1$}", "", i_us);
    }

}