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

    //--------
    println!("------------!!!");
    let mut c = 0;
    let loop_value = loop {
        println!("Hello from loop");
        if c == 3 {
            break 120;
        }
        c += 1;
    };
    println!("loop value = {}", loop_value);
    println!("------------!!!");

    let mut b=0;
    'main_loop: loop{
        println!("from main loop");

        'inner_loop: loop{
            b+=1;
            println!("inner loop");

            if b==3 {
                break 'main_loop;
            }
        }
    }
}
