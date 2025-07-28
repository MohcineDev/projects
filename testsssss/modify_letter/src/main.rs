fn main() {
    println!("{}", remove_letter_sensitive("Jojhn jis sljeepjjing", 'j'));
    println!(
        "{}",
        remove_letter_insensitive("JaimA ais swiaAmmingA", 'A')
    );
    println!("{}", swap_letter_case("byE bye", 'e'));
}

pub fn remove_letter_sensitive(s: &str, letter: char) -> String {
    let mut res = String::from("");

    for c in s.to_string().chars() {
        if c != letter {
            res.push(c);
        }
    }
    res
}

pub fn remove_letter_insensitive(s: &str, letter: char) -> String {
    let mut res = String::from("");

    let lower_letter = letter.to_lowercase().next().unwrap();
    let upper_letter = letter.to_uppercase().next().unwrap();

    for c in s.to_string().chars() {
        if c != lower_letter && c != upper_letter {
            res.push(c);
        }
    }
    res
}

pub fn swap_letter_case(s: &str, letter: char) -> String {
    let mut res = String::from("");

    let lower_letter = letter.to_lowercase().next().unwrap();
    let upper_letter = letter.to_uppercase().next().unwrap();

    for c in s.to_string().chars() {
        if c == lower_letter {
            res.push(upper_letter);
        } else if c == upper_letter {
            res.push(lower_letter);
        } else {
            res.push(c);
        }
    }
    res
}

/*

Joe is sleeping
Jim is swimming
bye byE

*/
