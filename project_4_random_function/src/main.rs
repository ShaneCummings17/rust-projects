// 4. Develop a program that uses a randomly generated number
// to select 1 of 3 (or more) functions to show the user.
use rand::Rng;

fn main() {
    let random_number = rand::thread_rng().gen_range(1..=3);
    match random_number {
        1 => random_1(),
        2 => random_2(),
        3 => random_3(),
        _ => ()
    }
}

fn random_1() {
    println!("You got the first random function!");
}

fn random_2() {
    println!("Woohoo! You got the second random function!");
}

fn random_3() {
    println!("Yeah! Third random function!");
}