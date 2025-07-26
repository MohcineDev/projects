pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut a = vec![];

    for elem in names {
        let n = elem.split_whitespace(); 

        let mut letter = String::from("");
        for e in n {

            let (first, _) = e.split_at(1);
            letter.push_str(first);
            letter.push_str(". ");
        }
        a.push(letter.trim().to_string());
    }

    a
}
/*
pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut a = vec![];

    for elem in names {
        let n = elem.split_whitespace();
        let mut counter = 0;

        let mut letter = String::from("");
        for e in n {
            counter += 1;

            let (first, _) = e.split_at(1);
            letter.push_str(first);
            letter.push_str(".");
            if counter < 2 {
                letter.push_str(" ");
            }
        }
        a.push(letter);
    }

    a
}

*/