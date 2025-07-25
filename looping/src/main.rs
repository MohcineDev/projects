use std::io;

fn main() {
    let mut counter = 0;

    loop {
    let mut answer = String::new();
        println!("I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?");
        counter += 1;

        io::stdin().read_line(&mut answer).expect("Failed to read line");
          
        if answer.trim().to_string() == "The letter e" {
            println!("Number of trials: {}", counter);
            break;
        }
    }
}
