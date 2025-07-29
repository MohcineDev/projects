pub fn capitalize_first(input: &str) -> String {
    let mut res: String = String::new();
    let mut index = 0;
    for c in input.chars() {
        if index == 0 {
            res.push_str(&c.to_uppercase().to_string());
            index += 1;
        } else {
            res.push_str(&c.to_string());
        }
    }
    res
}

pub fn title_case(input: &str) -> String {
    let mut res = String::new();
    println!("{}", input);
    let mut word = String::from("");
   // let   space = String::from("");
    let mut is_word = false;

    for c in input.chars() {
        if c.is_whitespace() {
            if is_word {
                res.push_str(&capitalize_first(&word));
                is_word = false;
                word = "".to_string();
            }
            res.push(c);
        } else {
            word.push(c);
            is_word = true;
        }
    }
    //last word
    if is_word {
        res.push_str(&capitalize_first(&word));
    }
    // for word in input.split_whitespace() {
    //     res.push_str(&capitalize_first(word));
    //     res.push_str(" ");
    // }

    res.trim().to_string()
}

pub fn change_case(input: &str) -> String {
    let mut res = String::new();
    for c in input.chars() {
        // println!("{}", c);
        if c == ' ' {
            res.push_str(" ");
        } else if c.is_uppercase() {
            res.push_str(&c.to_lowercase().to_string());
        } else if c.is_lowercase() {
            res.push_str(&c.to_uppercase().to_string());
        }
    }
    res
}
