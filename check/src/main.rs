// use temperature_conv::*;
// use speed_transformation::*;
// use groceries::*;
// use reverse_string::*;
// use find_factorial::*;
// use fibonacci2::*;
// use division_and_remainder::*;
// use tuples_refs::*;
// use borrow::*;

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

// fn main() {
//     let student = Student(20, "Pedro".to_string(), "Domingos".to_string());
//     println!("Student's first name: {}", first_name(&student));
//     println!("Student's last name: {}", last_name(&student));
//     println!("Student's id: {}", id(&student));
// }

// fn main() {
// 	let s = "hello";
// 	let s1 = "camelCase".to_string();
// 	let s2 = "olá!";

// 	println!("\tstr_len(\"{}\") = {}", s, str_len(s));
// 	println!("\tstr_len(\"{}\") = {}", s1, str_len(&s1));
// 	println!("\tstr_len(\"{}\") = {}", s2, str_len(s2));
// }

// use doubtful::*;

// fn main() {
//     let mut s: String = "Hello".to_owned();

//     println!("Before changing the string: {}", s);

//     doubtful(&mut s);

//     println!("After changing the string: {}", s);
// }

// use to_url::*;

// fn main() {
//     let s = "Hello, world!";
//     println!("'{}' parsed as an URL becomes '{}'", s, to_url(s));
// }

// use string_literals::*;

// fn main() {
//     println!("{}", is_empty(""));
//     println!("{}", is_ascii("rust"));
//     println!("{}", contains("rust", "ru"));
//     println!("{:?}", split_at("rust", 2));
//     println!("{}", find("rust", 'u'));
// }

// use name_initials::*;

// fn main() {
//     let names = vec!["Harry Potter", "Someone Else", "J. L.", "Barack Obama"];
//     println!("{:?}", initials(names));
// }

// use std::alloc::{self, GlobalAlloc, Layout};
// use std::sync::atomic::{AtomicUsize, Ordering};

// use name_initials::*;

// struct CounterAlloc {
//     counter: AtomicUsize,
// }

// #[allow(dead_code)] // incorrect false positive!
// impl CounterAlloc {
//     #[inline]
//     fn reset_counter(&self) {
//         self.counter.store(0, Ordering::SeqCst);
//     }

//     #[inline]
//     fn counter(&self) -> usize {
//         self.counter.load(Ordering::SeqCst)
//     }
// }

// unsafe impl GlobalAlloc for CounterAlloc {
//     unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
//         let ptr = unsafe { alloc::System.alloc(layout) };
//         self.counter.fetch_add(1, Ordering::SeqCst);
//         return ptr;
//     }

//     #[inline]
//     unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
//         unsafe {
//             alloc::System.dealloc(ptr, layout);
//         }
//     }
// }

// #[global_allocator]
// static ALLOCATOR: CounterAlloc = CounterAlloc {
//     counter: AtomicUsize::new(0),
// };

// fn main() {
//     let names = vec!["Harry Potter", "Someone Else", "J. L.", "Barack Obama"];
//     println!("{:?}", initials(names));
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     // Deliberately suboptimal solution to give the student some room for heap allocation
//     fn initials_sol(arr: Vec<&str>) -> Vec<String> {
//         arr.iter()
//             .map(|ele| {
//                 let mut names = ele.split_whitespace();
//                 let mut a = names.next().unwrap().chars().nth(0).unwrap().to_string();
//                 a.push_str(". ");
//                 let mut b = names.next().unwrap().chars().nth(0).unwrap().to_string();
//                 b.push_str(".");
//                 a.push_str(&b);
//                 a
//             })
//             .collect()
//     }

//     #[test]
//     fn test_memory_allocation() {
//         let test_value = ["Harry Potter", "Someone Else", "J. L.", "Barack Obama"];

//         let test = |f: &dyn Fn(Vec<&str>) -> Vec<String>| {
//             let test_value = test_value.to_vec();
//             ALLOCATOR.reset_counter();
//             f(test_value);
//             ALLOCATOR.counter()
//         };

//         let sol_alloc = test(&initials_sol);
//         let stu_alloc = test(&initials);

//         assert!(stu_alloc <= sol_alloc);
//     }

//     #[test]
//     fn test_function() {
//         let cases = [
//             (
//                 vec!["Harry Potter", "Someone Else", "J. L.", "Barack Obama"],
//                 vec!["H. P.", "S. E.", "J. L.", "B. O."],
//             ),
//             (
//                 vec![
//                     "James John",
//                     "David Joseph",
//                     "Matthew Brian",
//                     "Jacob Sousa",
//                     "Bruce Banner",
//                     "Scarlett Johansson",
//                     "Graydon Hoare",
//                 ],
//                 vec![
//                     "J. J.", "D. J.", "M. B.", "J. S.", "B. B.", "S. J.", "G. H.",
//                 ],
//             ),
//         ];

//         cases
//             .into_iter()
//             .for_each(|(n, e)| assert_eq!(initials(n), e));
//     }
// }

// use ownership::first_subword;

// fn main() {
//     let s1 = "helloWorld";
//     let s2 = "snake_case";
//     let s3 = "CamelCase";
//     let s4 = "just";

//      println!("first_subword({}) = {}", s1, first_subword(s1.to_owned()));
//     println!("first_subword({}) = {}", s2, first_subword(s2.to_owned()));
//     println!("first_subword({}) = {}", s3, first_subword(s3.to_owned()));
//     println!("first_subword({}) = {}", s4, first_subword(s4.to_owned()));
// }

// use copy::*;

// fn main() {
//     let a = "1 2 4 5 6".to_owned();
//     // let b = vec![1, 2, 4, 5];
//     let b = vec![1, 2, 4];
//     let c = 0;

//     println!("{:?}", nbr_function(c));
//     println!("{:?}", vec_function(b));
//     println!("{:?}", str_function(a));
// }

// use borrow_me_the_reference::*;

// fn main() {
//     let mut a = "bpp--o+er+++sskroi-++lcw".to_owned();
//     let mut b = [
//         "2+2".to_owned(),
//         "3+2".to_owned(),
//         "10-3".to_owned(),
//         "5+5".to_owned(),
//     ];

//     delete_and_backspace(&mut a);
//     do_operations(&mut b);

//     println!("{:?}", (a, b));
// }

// use tic_tac_toe::*;

// fn main() {
//     println!(
//         "{}",
//         tic_tac_toe([['O', 'X', 'O'], ['O', 'P', 'X'], ['X', '#', 'X']])
//     );
//     // tie
//     println!(
//         "{}",
//         tic_tac_toe([['X', 'O', 'O'], ['X', 'O', 'O'], ['#', 'O', 'X']])
//     );
//     // player O won

//     let diag = [['O', 'O', 'X'], ['O', 'X', 'O'], ['X', '#', 'X']];

//     println!("{}", tic_tac_toe(diag));
//     // player X won
// }

// use arrange_it::*;

// fn main() {
//     println!("{}", arrange_phrase("is2 Thi1s T4est 3a"));
// }
//This is a Test

// use changes::*;

// fn main() {
//     let mut lights = ["living_room", "bedroom", "rest_room"].map(Light::new);

//     println!("brightness = {}", lights[0].brightness);

//     change_brightness(&mut lights, "living_room", 200);

//     println!("new brightness = {}", lights[0].brightness);
// }

// use circle::*;

// fn main() {
//     let circle = Circle::new(500.0, 500.0, 150.0);
//     let circle1 = Circle {
//         center: Point(80.0, 115.0),
//         radius: 30.0,
//     };
//     let point_a = Point(1.0, 1.0);
//     let point_b = Point(0.0, 0.0);
//     println!("circle = {:?} area = {}", circle, circle.area());
//     println!("circle = {:?} diameter = {}", circle, circle.diameter());
//     println!("circle1 = {:?} diameter = {}", circle1, circle1.diameter());
//     println!(
//         "circle and circle1 intersect = {}",
//         circle.intersect(circle1)
//     );

//     println!(
//         "distance between {:?} and {:?} is {}",
//         point_a,
//         point_b,
//         point_a.distance(point_b)
//     );
// }

// use card_deck::*;

// fn main() {
//     let your_card = Card {
//         rank: Rank::random(),
//         suit: Suit::random(),
//     };

//     println!("Your card is {:?}", &your_card);

//     if card_deck::winner_card(&your_card) {
//         println!("You are the winner!");
//     }
// }


// #[cfg(test)]
// mod tests {
//     use super::*;

//     // We cannot truly test the randomness as there's no 100% accurate consistent way to prove through a predicate that it yields a truly random number

//     #[test]
//     fn test_winner() {
//         let winner = Card {
//             rank: Rank::Ace,
//             suit: Suit::Spade,
//         };

//         for rank in 1..14 {
//             for suit in 1..5 {
//                 let card = Card {
//                     rank: Rank::translate(rank),
//                     suit: Suit::translate(suit),
//                 };

//                 assert_eq!(card_deck::winner_card(&card), card == winner);
//             }
//         }
//     }
// }

// use arrays::*;

// fn main() {
//     // let a:[i32; 10] = (1..=10)._;
//     let b = [5;10];

//     // println!("The sum of the elements in {:?} is {}", a, sum(&a));
//     println!("The sum of the elements in {:?} is {}", b, sum(&b));
//     println!(
//         "Array of {} elements filled with 10 = {:?}",
//         thirtytwo_tens().len(),
//         thirtytwo_tens()
//     );
// }

 