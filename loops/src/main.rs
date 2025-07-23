fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        println!("The counter is {counter}");

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    //---
    println!("------------------");
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    println!("------------------");

    let a = [10, 20, 30, 40, 50];

    for elem in a {
        println!("the value is: {elem}");
    }
    println!("------------------");

      for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
