use std::io;
use rand::Rng;

fn main() {
    println!("Welcome to the guessing game!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The generated securet number is {}", secret_number);
    let mut user_guess = String::new();
    io::stdin()
        .read_line(&mut user_guess)
        .expect("Failed to get user input!");

    println!("Your guess is {}", user_guess);
}
