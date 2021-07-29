use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("That is the guess number game");

    let secret_number: u8 = rand::thread_rng().gen_range(1..101);

    println!("The secret number is {}", secret_number);

    println!("Please enter your guess");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u8 = guess
        .trim()
        .parse() // make inference and parse to an u8
        .expect("Please type a number between 1 and 100");

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}