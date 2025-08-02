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

//  use arrays::*;

// fn main() {
//     let a:Vec<i32>  = (1..=10).collect();
//     let b = [5;10];

//     println!("The sum of the elements in {:?} is {}", a, sum(&a));
//     println!("The sum of the elements in {:?} is {}", b, sum(&b));
//     println!(
//         "Array of {} elements filled with 10 = {:?}",
//         thirtytwo_tens().len(),
//         thirtytwo_tens()
//     );
// }

// use strings::*;

// fn main() {
// 	println!("length of {} = {}", "❤", char_length("❤"));
// 	println!("length of {} = {}", "形声字", char_length("形聲字"));
// 	println!("length of {} = {}", "change", char_length("change"));
// 	println!("length of {} = {}", "😍", char_length("😍"));
// }

// use capitalizing::*;

// fn main() {
//     println!("{}", capitalize_first("joe is missing"));
//     println!("{}", title_case("jill is leaving A"));
//     println!("{}", change_case("heLLo THere"));
// }

// fn main() {
//     // println!("{}", capitalize_first("joe is missing"));
//     // println!("{}", title_case("jill is leaving A"));
//     println!("{}", title_case("hello my\t\tname is carl"));
//     // println!("{}", change_case("heLLo THere"));
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_capitalize_first() {
//         assert_eq!(capitalize_first("hello"), "Hello");
//         assert_eq!(capitalize_first("this is working"), "This is working");
//     }

//     #[test]
//     fn test_title_case() {
//         assert_eq!(title_case("this is a title"), "This Is A Title");
//         assert_eq!(
//             title_case("hello my\t\tname is carl"),
//             "Hello My\t\tName Is Carl"
//         );
//     }

//     #[test]
//     fn test_change_case() {
//         assert_eq!(change_case("PROgraming"), "proGRAMING");
//         assert_eq!(change_case("heLLo THere"), "HEllO thERE");
//     }

//     #[test]
//     fn test_empty() {
//         assert_eq!(capitalize_first(""), "");
//         assert_eq!(title_case(""), "");
//         assert_eq!(change_case(""), "");
//     }
// }

// use edit_distance::*;

// fn main() {
//     let source = "alignment";
//     let target = "assignment";

//     println!(
//         "It's necessary to make {} change(s) to {:?} to get {:?}",
//         edit_distance(source, target),
//         source,
//         target
//     );
// }

// use simple_hash::*;

// const SENTENCE: &str = "this is a very basic sentence with only a few repetitions. once again this is very basic but it should be enough for basic tests";

// fn main() {
//     let words = SENTENCE.split_ascii_whitespace().collect::<Vec<_>>();
//     let frequency_count = word_frequency_counter(&words);

//     println!("{:?}", frequency_count);
//       println!("{}", nb_distinct_words(&frequency_count));
// }

// use bigger::*;
// use std::collections::HashMap;

// fn main() {
//     let hash = HashMap::from_iter([
//         ("Daniel", 122),
//         ("Ashley", 333),
//         ("Katie", 334),
//         ("Robert", 14),
//     ]);

//     println!(
//         "The biggest of the elements in the HashMap is {}",
//         bigger(hash)
//     );
// }

// use string_permutation::*;

// fn main() {
//     let word = "thought";
//     let word1 = "thougth";

//     println!(
//         "Is {:?} a permutation of {:?}? = {}",
//         word,
//         word1,
//         is_permutation(word, word1)
//     );
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_basic() {
//         assert!(is_permutation("abcde", "edbca"));
//         assert!(!is_permutation("avcde", "edbca"));
//         assert!(!is_permutation("cde", "edbca"));
//         assert!(is_permutation("code", "deco"));
//         assert!(!is_permutation("code", "deeco"));
//         assert!(!is_permutation("codde", "deeco"));
//     }

//     #[test]
//     fn test_repeating_characters() {
//         assert!(is_permutation("aab", "baa"));
//     }

//     #[test]
//     fn test_one_char() {
//         assert!(!is_permutation("a", "b"));
//         assert!(is_permutation("a", "a"));
//     }

//     #[test]
//     fn test_empty() {
//         assert!(is_permutation("", ""));
//     }

//     #[test]
//     fn test_special_characters() {
//         assert!(is_permutation("!#%@", "@%#!"));
//     }

//     #[test]
//     fn test_unicode() {
//         assert!(is_permutation("hello♥", "♥oelhl"));
//         assert!(!is_permutation("♥", "♥♥"));
//     }
// }

// use hashing::*;

// fn main() {
//     let v = [4, 7, 5, 2, 5, 1, 3];

//     println!("mean {}", mean(&v));
//     println!("median {}", median(&v));
//     println!("mode {}", mode(&v));
// }
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::f64;

//     #[inline]
//     fn approx_eq(a: f64, b: f64) -> bool {
//         (a - b).abs() < f64::EPSILON
//     }

//     #[test]
//     fn test_mean() {
//         let v = [4, 7, 5, 2, 5, 1, 3];
//         assert!(approx_eq(mean(&v), 3.857142857142857));
//     }

//     #[test]
//     fn test_median() {
//         assert_eq!(median(&[4, 7, 5, 2, 5, 1, 3]), 4);
//         assert_eq!(median(&[2, 1, 5, 2, 7, 4]), 3);
//         assert_eq!(median(&[1, 7, 5, 5, 6, 4]), 5);
//     }

//     #[test]
//     fn test_mode() {
//         let v = [4, 7, 5, 2, 5, 1, 3];
//         assert_eq!(mode(&v), 5);
//     }
// }

// use collect::*;

// fn main() {
//     let mut v = [3, 2, 4, 5, 1, 7];
//     let mut v_clone = v;

//     bubble_sort(&mut v);
//     println!("{:?}", v);

//     v_clone.sort_unstable();
//     println!("{:?}", v_clone);
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_ordering() {
//         let mut v = [3, 2, 4, 5, 1, 7, 9, 8];
//         let mut v_clone = v;

//         v_clone.sort_unstable();
//         bubble_sort(&mut v);

//         assert_eq!(v, v_clone);
//     }
// }

// use unwrap_or_expect::*;

// fn main() {
//     println!("{}", fetch_data(Ok("server1.com"), Security::Warning));
//     println!("{}", fetch_data(Err("server.com"), Security::Warning));
//     println!("{}", fetch_data(Err("server2.com"), Security::NotFound));

//     // Panics with no custom message
//     // fetch_data(Err("ERROR CRITICAL"), Security::Unknown);

//     // Panics with the message "ERROR: program stops"
//     // fetch_data(Err("server.com"), Security::Message);

//     // Panics with the message "malicious_server.com"
//     // fetch_data(Ok("malicious_server.com"), Security::UnexpectedUrl);
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     #[should_panic(expected = "ERROR: program stops")]
//     fn test_expect() {
//         fetch_data(Err(""), Security::Message);
//     }
//     #[test]
//     #[should_panic(expected = "called `Result::unwrap()` on an `Err` value: \"ERROR CRITICAL\"")]
//     fn test_unwrap() {
//         fetch_data(Err("ERROR CRITICAL"), Security::Unknown);
//     }
//     #[test]
//     #[should_panic(expected = "malicious_server.com")]
//     fn test_unwrap_err() {
//         fetch_data(Ok("malicious_server.com"), Security::UnexpectedUrl);
//     }
//     #[test]
//     fn test_unwrap_or() {
//         assert_eq!(
//             fetch_data(Err(""), Security::Warning),
//             "WARNING: check the server".to_string()
//         );
//     }
//     #[test]
//     fn test_unwrap_or_else() {
//         assert_eq!(
//             fetch_data(Err("another_server.com"), Security::NotFound),
//             "Not found: another_server.com".to_string()
//         );
//     }
//     #[test]
//     fn test_ok() {
//         assert_eq!(
//             fetch_data(Ok("server.com"), Security::Message),
//             "server.com"
//         );
//         assert_eq!(
//             fetch_data(Ok("server.com"), Security::Warning),
//             "server.com"
//         );
//         assert_eq!(
//             fetch_data(Ok("server.com"), Security::NotFound),
//             "server.com"
//         );
//         assert_eq!(
//             fetch_data(Ok("server.com"), Security::Unknown),
//             "server.com"
//         );
//     }
// }
// use panic::*;
// use std::fs::{self, File};

// fn main() {
//     let filename = "created.txt";
//     File::create(filename).unwrap();

//     println!("{:?}", open_file(filename));

//     fs::remove_file(filename).unwrap();

//     // this should panic!
//     open_file(filename);
// }

// use std::fs;

// fn main() {
//     let path = "a.txt";

//     handling::open_or_create(&path, "content to be written");

//     let contents = fs::read_to_string(path).unwrap();
//     println!("{}", contents);
// }

// fn main() {
//     ["hello there", "", "you are stupid", "stupid"]
//         .into_iter()
//         .for_each(|m| println!("{:?}", profanity_filter::check_ms(m)));
// }

// use question_mark::*;

// fn main() {
//     let a = One {
//         first_layer: Some(Two {
//             second_layer: Some(Three {
//                 third_layer: Some(Four {
//                     fourth_layer: Some(1000)
//                 })
//             })
//         })
//     };

//     println!("{:?}", a.get_fourth_layer());
// }

// use banner::*;
// use std::{collections::HashMap, sync::LazyLock};

// fn main() {
//     let mut handler = FlagsHandler {
//         flags: HashMap::new(),
//     };

//     let d = Flag::opt_flag("division", "divides the values, formula (a / b)");
//     let r = Flag::opt_flag(
//         "remainder",
//         "remainder of the division between two values, formula (a % b)",
//     );

//     handler.add_flag(d, div);
//     handler.add_flag(r, rem);

//     println!("{:?}", handler.exec_func("-d", &["1.0", "2.0"]));

//     println!("{:?}", handler.exec_func("-r", &["2.0", "2.0"]));

//     println!("{:?}", handler.exec_func("--division", &["a", "2.0"]));

//     println!("{:?}", handler.exec_func("--remainder", &["2.0", "fd"]));
// }

// static HANDLER: LazyLock<FlagsHandler> = LazyLock::new(|| {
//     let mut handler = FlagsHandler {
//         flags: HashMap::new(),
//     };

//     handler.add_flag(Flag::opt_flag("division", "divides two numbers"), div);
//     handler.add_flag(
//         Flag::opt_flag(
//             "remainder",
//             "gives the remainder of the division between two numbers",
//         ),
//         rem,
//     );

//     handler
// });

// #[test]
// fn test_simple() {
//     for a in ["-d", "--division"] {
//         assert_eq!(HANDLER.exec_func(a, &["1.0", "2.0"]), Ok("0.5".to_owned()));
//     }

//     for a in ["-r", "--remainder"] {
//         assert_eq!(HANDLER.exec_func(a, &["2.0", "2.0"]), Ok("0".to_owned()));
//     }

//     for a in ["-d", "--division"] {
//         assert_eq!(
//             HANDLER.exec_func(a, &["12.323", "212.32"]),
//             Ok("0.058039751318764134".to_owned())
//         );
//     }

//     for a in ["-r", "--remainder"] {
//         assert_eq!(
//             HANDLER.exec_func(a, &["12.323", "212.32"]),
//             Ok("12.323".to_owned())
//         );
//     }
// }

// #[test]
// fn test_edge_cases() {
//     for a in ["-d", "--division"] {
//         assert_eq!(
//             HANDLER.exec_func(a, &["a", "2.0"]),
//             Err("invalid float literal".to_owned())
//         );
//     }

//     for a in ["-r", "--remainder"] {
//         assert_eq!(
//             HANDLER.exec_func(a, &["2.0", "f"]),
//             Err("invalid float literal".to_owned())
//         );
//     }

//     for a in ["-d", "--division"] {
//         assert_eq!(HANDLER.exec_func(a, &["1.0", "0.0"]), Ok("inf".to_owned()));
//     }

//     for a in ["-r", "--remainder"] {
//         assert_eq!(HANDLER.exec_func(a, &["1.0", "0.0"]), Ok("NaN".to_owned()));
//     }
// }

//  use cipher::*;

// fn main() {
//     println!("{:?}", cipher("1Hello 2world!", "1Svool 2dliow!"));
//      println!("{:?}", cipher("1Hello 2world!", "svool"));
// }

// this will be the structure that wil handle the errors

// use error_types::*;
// use chrono::Utc;
// fn main() {
//     let mut form_output = Form {
//         name: "Lee".to_owned(),
//         password: "qwqwsa1dty_".to_owned(),
//     };

//     println!("{:?}", form_output);
//     println!("{:?}", form_output.validate());

//     form_output.name = "".to_owned();
//     println!("{:?}", form_output.validate());

//     form_output.name = "as".to_owned();
//     form_output.password = "dty_1".to_owned();
//     println!("{:?}", form_output.validate());

//     form_output.password = "asdasASd(_".to_owned();
//     println!("{:?}", form_output.validate());

//     form_output.password = "asdasASd123SA".to_owned();
//     println!("{:?}", form_output.validate());
// }

// #[test]
// fn test_error_type() {
//     let cases = [
//         (
//             Form {
//                 name: "Katy".to_owned(),
//                 password: "qwTw12&%$3sa1dty_".to_owned(),
//             },
//             Ok(()),
//         ),
//         (
//             Form {
//                 name: "".to_owned(),
//                 password: "qwTw12&%$3sa1dty_".to_owned(),
//             },
//             Err(FormError {
//                 form_values: ("name", "".to_owned()),
//                 date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
//                 err: "Username is empty",
//             }),
//         ),
//         (
//             Form {
//                 name: "Someone".to_owned(),
//                 password: "12345".to_owned(),
//             },
//             Err(FormError {
//                 form_values: ("password", "12345".to_owned()),
//                 date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
//                 err: "Password should be at least 8 characters long",
//             }),
//         ),
//         (
//             Form {
//                 name: "Someone".to_owned(),
//                 password: "sdASDsrW".to_owned(),
//             },
//             Err(FormError {
//                 form_values: ("password", "sdASDsrW".to_owned()),
//                 date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
//                 err: "Password should be a combination of ASCII numbers, letters and symbols",
//             }),
//         ),
//         (
//             Form {
//                 name: "Someone".to_owned(),
//                 password: "dsGE1SAD213".to_owned(),
//             },
//             Err(FormError {
//                 form_values: ("password", "dsGE1SAD213".to_owned()),
//                 date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
//                 err: "Password should be a combination of ASCII numbers, letters and symbols",
//             }),
//         ),
//         (
//             Form {
//                 name: "Someone".to_owned(),
//                 password: "dsaSD&%DF!?=".to_owned(),
//             },
//             Err(FormError {
//                 form_values: ("password", String::from("dsaSD&%DF!?=")),
//                 date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
//                 err: "Password should be a combination of ASCII numbers, letters and symbols",
//             }),
//         ),
//     ];

//     for (form, expected) in cases {
//         assert_eq!(form.validate(), expected);
//     }
// }

// use std::{fs::File, io::Write};

// use boxing_todo::TodoList;

// fn main() {
//     let files = [
//         (
//             "todo.json",
//             r#"{
//                 "title" : "TODO LIST FOR PISCINE RUST",
//                 "tasks": [
//                     { "id": 0, "description": "do this", "level": 0 },
//                     { "id": 1, "description": "do that", "level": 5 }
//                 ]
//             }"#,
//         ),
//         (
//             "todo_empty.json",
//             r#"{
//                 "title" : "TODO LIST FOR PISCINE RUST",
//                 "tasks": []
//             }"#,
//         ),
//         (
//             "malformed_object.json",
//             r#"{
//                 "something": ,
//             }"#,
//         ),
//     ];

//     for (name, content) in files {
//         File::create(name)
//             .unwrap()
//             .write(content.as_bytes())
//             .unwrap();

//         let todos = TodoList::get_todo(name);
//         match todos {
//             Ok(list) => println!("{:?}", list),
//             Err(e) => {
//                 println!("{}: {:?}", e.to_string(), e.source());
//             }
//         }
//     }
// }

// use middle_day::*;

// fn main() {
//     println!("{:?}", middle_day(1022));
// }

// use does_it_fit::*;
// // use does_it_fit::areas_volumes::GeometricalShapes;
// // use does_it_fit::areas_volumes::GeometricalVolumes;

// fn main() {
//     println!(
//         "Do 100 rectangles (2x1) fit in a 2 by 4 square? {}",
//         area_fit((2, 4), GeometricalShapes::Rectangle, 100, (2, 1))
//     );
//     println!(
//         "Do 3 triangles (5 base and 3 height) fit in a 5 by 5 square? {}",
//         area_fit((5, 5), GeometricalShapes::Triangle, 3, (5, 3))
//     );
//     println!(
//         "Do 3 spheres (2 radius) fit in a 5 by 5 by 5 box? {}",
//         volume_fit((5, 5, 5), GeometricalVolumes::Sphere, 3, (2, 0, 0))
//     );
//     println!(
//         "Does 1 parallelepiped (6 base, 7 height and depth 4) fit in a 5 by 7 by 5 parallelepiped? {}",
//         volume_fit((5, 7, 5), GeometricalVolumes::Parallelepiped, 1, (6, 7, 4))
//     );
// }

// #[test]
// fn no_volumes_shapes() {
//     assert!(area_fit((2, 5), GeometricalShapes::Circle, 0, (2, 1)));
//     assert!(area_fit((2, 2), GeometricalShapes::Rectangle, 0, (6, 10)));
//     assert!(volume_fit(
//         (2, 5, 3),
//         GeometricalVolumes::Cone,
//         0,
//         (1, 1, 1)
//     ));
//     assert!(volume_fit(
//         (3, 5, 7),
//         GeometricalVolumes::Parallelepiped,
//         0,
//         (2, 6, 3)
//     ));
// }

// #[test]
// fn equal_size() {
//     assert!(area_fit((2, 5), GeometricalShapes::Square, 1, (2, 5)));
//     assert!(volume_fit(
//         (3, 1, 4),
//         GeometricalVolumes::Cube,
//         1,
//         (1, 3, 4)
//     ));
// }

// #[test]
// fn all_shapes() {
//     assert!(!area_fit((3, 5), GeometricalShapes::Circle, 2, (2, 0)));
//     assert!(area_fit((8, 6), GeometricalShapes::Triangle, 5, (5, 2)));
//     assert!(area_fit((7, 3), GeometricalShapes::Rectangle, 2, (2, 4)));
//     assert!(area_fit((5, 5), GeometricalShapes::Square, 1, (2, 4)));
// }

// #[test]
// fn all_volumes() {
//     assert!(volume_fit(
//         (5, 6, 3),
//         GeometricalVolumes::Cube,
//         2,
//         (3, 3, 4)
//     ));
//     assert!(!volume_fit(
//         (7, 4, 4),
//         GeometricalVolumes::Cone,
//         1,
//         (8, 2, 0)
//     ));
//     assert!(volume_fit(
//         (2, 5, 3),
//         GeometricalVolumes::Sphere,
//         1,
//         (1, 1, 1)
//     ));
//     assert!(!volume_fit(
//         (2, 5, 3),
//         GeometricalVolumes::Parallelepiped,
//         31,
//         (1, 1, 1)
//     ));
//     assert!(volume_fit(
//         (7, 5, 3),
//         GeometricalVolumes::TriangularPyramid,
//         3,
//         (3, 2, 1)
//     ));
// }

// use macro_calculator::*;

// fn main() {
//     let foods = [
//         Food {
//             name: "big mac".to_owned(),
//             calories: ("2133.84kJ".to_owned(), "510kcal".to_owned()),
//             proteins: 27.,
//             fats: 26.,
//             carbs: 41.,
//             nbr_of_portions: 2.,
//         },
//         Food {
//             name: "pizza margherita".to_owned(),
//             calories: ("1500.59kJ".to_owned(), "358.65kcal".to_owned()),
//             proteins: 13.89,
//             fats: 11.21,
//             carbs: 49.07,
//             nbr_of_portions: 4.9,
//         },
//     ];

//     println!("{:#}", calculate_macros(&foods));
// }

// #[test]
// fn testing_macros_values() {
//     let a = Food {
//         name: "light milk".to_owned(),
//         calories: ("148kJ".to_owned(), "35kcal".to_owned()),
//         proteins: 3.5,
//         fats: 0.1,
//         carbs: 5.0,
//         nbr_of_portions: 0.7,
//     };
//     let b = Food {
//         name: "oat cookies".to_owned(),
//         calories: ("1996kJ".to_owned(), "477kcal".to_owned()),
//         proteins: 8.2,
//         fats: 21.0,
//         carbs: 60.4,
//         nbr_of_portions: 1.2,
//     };

//     let macros = calculate_macros(&[a, b]);

//     assert_eq!(macros["cals"].as_f64().unwrap(), 596.9);
//     assert_eq!(macros["carbs"].as_f64().unwrap(), 75.98);
//     assert_eq!(macros["proteins"].as_f64().unwrap(), 12.29);
//     assert_eq!(macros["fats"].as_f64().unwrap(), 25.27);
// }

// #[test]
// fn testing_no_food() {
//     let macros = calculate_macros(&[]);

//     assert_eq!(macros["cals"].as_f64().unwrap(), 0.0);
//     assert_eq!(macros["carbs"].as_f64().unwrap(), 0.0);
//     assert_eq!(macros["proteins"].as_f64().unwrap(), 0.0);
//     assert_eq!(macros["fats"].as_f64().unwrap(), 0.0);
// }

// #[test]
// fn big_values() {
//     let macros = calculate_macros(&[
//         Food {
//             name: "big mac".to_owned(),
//             calories: ("2133.84kJ".to_owned(), "510kcal".to_owned()),
//             proteins: 27.0,
//             fats: 26.0,
//             carbs: 41.0,
//             nbr_of_portions: 2.0,
//         },
//         Food {
//             name: "pizza margherita".to_owned(),
//             calories: ("1500.59kJ".to_owned(), "358.65kcal".to_owned()),
//             proteins: 13.89,
//             fats: 11.21,
//             carbs: 49.07,
//             nbr_of_portions: 4.9,
//         },
//     ]);

//     assert_eq!(macros["cals"].as_f64().unwrap(), 2777.39);
//     assert_eq!(macros["carbs"].as_f64().unwrap(), 322.44);
//     assert_eq!(macros["proteins"].as_f64().unwrap(), 122.06);
//     assert_eq!(macros["fats"].as_f64().unwrap(), 106.93);
// }

// use shopping_mall::*;
// use shopping_mall::mall::Floor;
// use shopping_mall::mall::Guard;
// use shopping_mall::mall::Mall;
// use shopping_mall::mall::Store;
// use shopping_mall::mall::Employee;

// fn main() {
//     let mut mall = Mall::new(
//         "La Vie Funchal",
//         [
//             (
//                 "John Oliver",
//                 Guard {
//                     age: 34,
//                     years_experience: 7,
//                 },
//             ),
//             (
//                 "Bob Schumacher",
//                 Guard {
//                     age: 53,
//                     years_experience: 15,
//                 },
//             ),
//         ]
//         .into(),
//         [
//             (
//                 "Ground Floor",
//                 Floor::new(
//                     [
//                         (
//                             "Footzo",
//                             Store::new(
//                                 [
//                                     (
//                                         "Finbar Haines",
//                                         Employee {
//                                             age: 36,
//                                             working_hours: (9, 14),
//                                             salary: 650.88,
//                                         },
//                                     ),
//                                     (
//                                         "Sienna-Rose Penn",
//                                         Employee {
//                                             age: 26,
//                                             working_hours: (9, 22),
//                                             salary: 1000.43,
//                                         },
//                                     ),
//                                 ]
//                                 .into(),
//                                 50,
//                             ),
//                         ),
//                         (
//                             "Swashion",
//                             Store::new(
//                                 [
//                                     (
//                                         "Abdallah Stafford",
//                                         Employee {
//                                             age: 54,
//                                             working_hours: (8, 22),
//                                             salary: 1234.21,
//                                         },
//                                     ),
//                                     (
//                                         "Marian Snyder",
//                                         Employee {
//                                             age: 21,
//                                             working_hours: (8, 14),
//                                             salary: 831.9,
//                                         },
//                                     ),
//                                 ]
//                                 .into(),
//                                 43,
//                             ),
//                         ),
//                     ]
//                     .into(),
//                     300,
//                 ),
//             ),
//             (
//                 "Supermarket",
//                 Floor::new(
//                     [(
//                         "Pretail",
//                         Store::new(
//                             [
//                                 (
//                                     "Yara Wickens",
//                                     Employee {
//                                         age: 39,
//                                         working_hours: (9, 14),
//                                         salary: 853.42,
//                                     },
//                                 ),
//                                 (
//                                     "Indiana Baxter",
//                                     Employee {
//                                         age: 33,
//                                         working_hours: (13, 20),
//                                         salary: 991.71,
//                                     },
//                                 ),
//                                 (
//                                     "Jadine Page",
//                                     Employee {
//                                         age: 48,
//                                         working_hours: (13, 20),
//                                         salary: 743.21,
//                                     },
//                                 ),
//                                 (
//                                     "Tyler Hunt",
//                                     Employee {
//                                         age: 63,
//                                         working_hours: (13, 20),
//                                         salary: 668.25,
//                                     },
//                                 ),
//                                 (
//                                     "Mohsin Mcgee",
//                                     Employee {
//                                         age: 30,
//                                         working_hours: (19, 24),
//                                         salary: 703.83,
//                                     },
//                                 ),
//                                 (
//                                     "Antoine Goulding",
//                                     Employee {
//                                         age: 45,
//                                         working_hours: (19, 24),
//                                         salary: 697.12,
//                                     },
//                                 ),
//                                 (
//                                     "Mark Barnard",
//                                     Employee {
//                                         age: 53,
//                                         working_hours: (19, 24),
//                                         salary: 788.81,
//                                     },
//                                 ),
//                             ]
//                             .into(),
//                             950,
//                         ),
//                     )]
//                     .into(),
//                     1000,
//                 ),
//             ),
//         ]
//         .into(),
//     );

//     // returns the biggest store
//     println!("Biggest store: {:#?}", biggest_store(&mall));

//     // returns the list with the highest paid employees
//     // println!("Highest paid employee: {:#?}", highest_paid_employee(&mall));

//     // returns the number of employees
//     // println!("Number of employees: {}", nbr_of_employees(&mall));

//     // checks if it is needed to add securities
//     // check_for_securities(
//     //     &mut mall,
//     //     [
//     //         (
//     //             "Peter Solomons",
//     //             Guard {
//     //                 age: 45,
//     //                 years_experience: 20,
//     //             },
//     //         ),
//     //         (
//     //             "William Charles",
//     //             Guard {
//     //                 age: 32,
//     //                 years_experience: 10,
//     //             },
//     //         ),
//     //         (
//     //             "Leonardo Changretta",
//     //             Guard {
//     //                 age: 23,
//     //                 years_experience: 0,
//     //             },
//     //         ),
//     //         (
//     //             "Vlad Levi",
//     //             Guard {
//     //                 age: 38,
//     //                 years_experience: 8,
//     //             },
//     //         ),
//     //         (
//     //             "Faruk Berkai",
//     //             Guard {
//     //                 age: 40,
//     //                 years_experience: 15,
//     //             },
//     //         ),
//     //         (
//     //             "Christopher Smith",
//     //             Guard {
//     //                 age: 35,
//     //                 years_experience: 9,
//     //             },
//     //         ),
//     //         (
//     //             "Jason Mackie",
//     //             Guard {
//     //                 age: 26,
//     //                 years_experience: 2,
//     //             },
//     //         ),
//     //         (
//     //             "Kenzie Mair",
//     //             Guard {
//     //                 age: 34,
//     //                 years_experience: 8,
//     //             },
//     //         ),
//     //         (
//     //             "Bentley Larson",
//     //             Guard {
//     //                 age: 33,
//     //                 years_experience: 10,
//     //             },
//     //         ),
//     //         (
//     //             "Ray Storey",
//     //             Guard {
//     //                 age: 37,
//     //                 years_experience: 12,
//     //             },
//     //         ),
//     //     ]
//     //     .map(|(n, d)| (n.to_owned(), d))
//     //     .into(),
//     // );

//     // raises or cuts the salary of every employee
//     // cut_or_raise(&mut mall);

//     // println!("{:#?}", mall);
// }


// use stars::stars;

// fn main() {
//     println!("{}", stars(1));
//     println!("{}", stars(4));
//     println!("{}", stars(5));
// }

// use scores::*;

// fn main() {
//     println!("{}", score("a"));
//     println!("{}", score("ã ê Á?"));
//     println!("{}", score("ThiS is A Test"));
// }

// use searching::*;

// fn main() {
//     let ar = [1, 3, 4, 6, 8, 9, 11, 8];
//     let f = search(&ar, 8);
//     println!(
//         "the element 8 is last in the position {:?} in the array {:?}",
//         f, ar
//     );
// }

// use searching::*;

// #[test]
// fn test_search_a_value_in_an_array() {
//     assert_eq!(search(&[6], 6), Some(0));
//     assert_eq!(search(&[1, 2], 1), Some(0));
//     assert_eq!(search(&[1, 2], 2), Some(1));
// }
// #[test]
// fn test_middle_of_an_array() {
//     assert_eq!(search(&[1, 3, 4, 6, 8, 9, 11], 6), Some(3));
// }

// #[test]
// fn test_beginning_of_an_array() {
//     assert_eq!(search(&[1, 3, 4, 6, 8, 9, 11], 1), Some(0));
// }

// #[test]
// fn test_end_of_an_array() {
//     assert_eq!(search(&[1, 3, 4, 6, 8, 9, 11], 11), Some(6));
// }

// #[test]
// fn test_long_array() {
//     assert_eq!(
//         search(&[1, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 634], 144),
//         Some(9)
//     );
//     assert_eq!(
//         search(&[1, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377], 21),
//         Some(5)
//     );
// }

// #[test]
// fn test_with_duplicates() {
//     assert_eq!(search(&[1, 3, 4, 6, 8, 9, 11, 1], 1), Some(7));
//     assert_eq!(search(&[1, 3, 9, 6, 8, 9, 11], 9), Some(5));
// }

// #[test]
// fn test_value_is_not_included() {
//     assert_eq!(search(&[1, 3, 4, 6, 8, 9, 11], 7), None);
//     assert_eq!(search(&[1, 3, 4, 6, 8, 9, 11], 0), None);
//     assert_eq!(search(&[1, 3, 4, 6, 8, 9, 11], 13), None);
//     assert_eq!(search(&[], 1), None);
// }
 
use ordinal::*;

fn main() {
    println!("{}", num_to_ordinal(1));
    println!("{}", num_to_ordinal(22));
    println!("{}", num_to_ordinal(11));
    println!("{}", num_to_ordinal(12));
    println!("{}", num_to_ordinal(43));
    println!("{}", num_to_ordinal(47));
}

