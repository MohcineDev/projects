// use temperature_conv::*;
// use speed_transformation::*;
// use groceries::*;
// use reverse_string::*;
// use find_factorial::*;
// use fibonacci2::*;
// use division_and_remainder::*;
use tuples_refs::*;

// fn main() {
//     println!("{} F = {} C", -459.67, fahrenheit_to_celsius(-459.67));
//     println!("{} C = {} F", 0.0, celsius_to_fahrenheit(0.0));
// }

// fn main() {
//     let km_h = 100.0;
//     let m_s = km_per_hour_to_meters_per_second(km_h);
//     println!("{} km/h is equivalent to {} m/s", km_h, m_s);
// }

// fn main() {
//     let mut groceries = vec![
//         "yogurt".to_string(),
//         "panettone".to_string(),
//         "bread".to_string(),
//         "cheese".to_string(),
//     ];
//     insert(&mut groceries, String::from("nuts"));
//     println!("groceries = {:?}", &groceries);
//     println!("groceries[1] = {:?}", at_index(&groceries, 1));
// }

// fn main() {
//     println!("{}", rev_str("Hello, world!"));
//     println!("{}", rev_str("Hello, my name is Roman"));
//     println!("{}", rev_str("I have a nice car!"));
//     println!("{}", rev_str("How old are You"));
//     println!("{}", rev_str("ex: this is an example água"));
// }

// fn main() {
//     println!("The factorial of 0 = {}", factorial(0));
//     println!("The factorial of 1 = {}", factorial(1));
//      println!("The factorial of 5 = {}", factorial(5));
//       println!("The factorial of 10 = {}", factorial(10));
//       println!("The factorial of 19 = {}", factorial(19));
// }

// fn main() {
//     println!(
//         "The element in the position {} in fibonacci series is {}",
//         2,
//         fibonacci(2)
//     );
//     println!(
//         "The element in the position {} in fibonacci series is {}",
//         4,
//         fibonacci(4)
//     );
//     println!(
//         "The element in the position {} in fibonacci series is {}",
//         22,
//         fibonacci(22)
//     );
//     println!(
//         "The element in the position {} in fibonacci series is {}",
//         20,
//         fibonacci(20)
//     );
// }


// fn main() {
//     let x = 9;
//     let y = 4;
//     let (division, remainder) = divide(x, y);
//     println!(
//         "{}/{}: division = {}, remainder = {}",
//         x, y, division, remainder
//     );
// }


fn main() {
    let student = Student(20, "Pedro".to_string(), "Domingos".to_string());
    println!("Student's first name: {}", first_name(&student));
    println!("Student's last name: {}", last_name(&student));
    println!("Student's id: {}", id(&student));
}