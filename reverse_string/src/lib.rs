
pub fn rev_str(input: &str) -> String {
    let mut rev_input = String::new();
    rev_input = input.chars().rev().collect();

    return  rev_input;
}
// pub fn rev_str(input: &str) -> String {
//     let mut input_len = input.len();
//     let mut rev_input = String::new();
//     // let mut chars: Vec<char> = input.chars();

//     // rev_input = input.to_string();

//     let mut length = input_len;
    
//     while input_len > 0 {
//         println!("{}", &rev_input[length - 1..length]);
//     let mut rev_input = String::new();
        
//         let my_char = &rev_input[length - 1..length];
 
//         rev_input.push_str(my_char);
//         length -= 1;

//         if input_len >= 0 {
//             input_len -= 1;
//         }
//     }

//     return rev_input;
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
