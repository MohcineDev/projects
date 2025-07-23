use std::io;
//io inpput/output library from standard library
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is : {secret_number}");

    println!("Pleae input your guess.");

    //mut makes the variable mutable - add it before the var name
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
