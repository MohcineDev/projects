use std::io;
//io inpput/output library from standard library

fn main() {
    println!("Guess the number!");

    println!("Pleae input your guess.");

    //mut makes the variable mutable - add it before the var name
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
