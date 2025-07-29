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

use collect::*;

fn main() {
    let mut v = [3, 2, 4, 5, 1, 7];
    let mut v_clone = v;

    bubble_sort(&mut v);
    println!("{:?}", v);

    v_clone.sort_unstable();
    println!("{:?}", v_clone);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ordering() {
        let mut v = [3, 2, 4, 5, 1, 7, 9, 8];
        let mut v_clone = v;

        v_clone.sort_unstable();
        bubble_sort(&mut v);

        assert_eq!(v, v_clone);
    }
}