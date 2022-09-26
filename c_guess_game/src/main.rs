/* Use the standard library */
use std::io;

/* Use external library */
use rand::Rng;

/* Point of entry */
fn main() {

    println!("Guess the number!");

    /* Random number generation */
    let random_number = rand::thread_rng().gen_range(1..=100);

    println!("Random number: {random_number}");

    println!("Please input your guess: ");

    /*  Variable
        In rust variables are not mutable by default
        let i = 1; // immutable
        let mut i = 1; // mutable
    */
    let mut guess = String::new();

    /* Input text */
    io::stdin()
        /* Write a string referring to a variable */
        .read_line(&mut guess)
        /* Message in case of error */
        .expect("Failed to read line");

    /* Output */
    println!("You guessed: {guess} ");

}