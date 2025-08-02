pub fn num_to_ordinal(x: u32) -> String {
    let mut res = String::from("");

    let c = &x.to_string();
    let len: usize = c.len();
    let mut i: usize = 0;
    let mut last_char: char = '_';

    for chss in c.chars() {
        if i == len - 1 {
            last_char = chss;
        }
        i += 1;
    }

    if last_char == '1' && x != 11 {
        res.push_str(&x.to_string());
        res.push_str("st");
    } else if last_char == '2' && x != 12 {
        res.push_str(&x.to_string());
        res.push_str("nd");
    } else if last_char == '3' && x != 13 {
        res.push_str(&x.to_string());
        res.push_str("rd");
    } else if last_char == '7' || (11..=19).contains(&x) || last_char == '0'{
        res.push_str(&x.to_string());
        res.push_str("th");
    }

    if res == "" {
        return String::from("0th");
    } else {
        res
    }
}
