fn main() {
    println!("Hello, world!");

    another_function(88);

    let five = five();
    println!("the value of five is : {five}");
}

fn another_function(x: u8) {
    println!("the value of x is : {x}");
}

fn five() -> u8 {
     5
}
