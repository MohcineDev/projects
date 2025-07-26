pub fn delete_and_backspace(s: &mut String) {
    // let mut res: Vec<String> = Vec::new();
    let mut res: Vec<String> = vec![];
    let mut counter = 0;

    for elem in s.chars() {
        if elem != '+' && counter > 0 {
            // res.pop();
            counter -= 1;
            continue;
        }
        if elem != '-' && elem != '+' {
            res.push(elem.to_string());
        }

        if elem == '-' {
            res.pop();
        }
        if elem == '+' {
            counter += 1;
        }
    }
    *s = res.join("");
}

pub fn do_operations(v: &mut [String]) {
    
    for elem in v {
        let mut res = 0;
        if elem.contains("+") {
            for a in elem.split("+") {
                let num: i32 = a.parse().expect("something wrong");
                res += num;
            }
        } else if elem.contains("-") {
            for a in elem.split("-") {
                let num: i32 = a.parse().expect("something wrong");
                if res > 0 {
                    res -= num;
                } else {
                    res += num;
                }
            }
        }

        *elem = res.to_string();
    }
}
