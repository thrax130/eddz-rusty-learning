use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
    println!("Guess the numbers");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is {secret_number}");
    println!("Please input your guess:");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess : u32 = guess.trim().parse().expect("Input could not be parsed as uint32");
    println!("You guessed {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too high!"),
        Ordering::Equal => println!("You win")
    }
}
