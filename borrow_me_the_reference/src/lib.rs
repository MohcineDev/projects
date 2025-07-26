pub fn delete_and_backspace(s: &mut String) {
    let mut res = String::from("");
    let mut counter = 0;

    for elem in s.chars() {
        if elem != '+' && counter > 0 {
            counter -= 1;
            continue;
        }
        if elem != '+' && elem != '-' {
            res.push(elem);
        }
        if elem == '+' {
            counter += 1;
        }
        if elem == '-' {
            res.pop();
        }
    }
    *s = String::from(res);
}
// ("borrow", ["4", "5", "7", "10"])

pub fn do_operations(v: &mut [String]) {

    for elem in v {
    
        let mut res = 0;
        let  parts: Vec<&str>;

        if elem.contains("+") {
            parts = elem.split("+").collect();
    
            for el in parts {
                let first: i32 = el.parse().expect("something wrong");
                res += first;
            }
        } else if elem.contains("-") {
            parts = elem.split("-").collect();
    
            for el in parts {
                let first: i32 = el.parse().expect("something wrong");
                if res > 0 {
                    res -= first;
                } else {
                    res += first;
                }
            }
        }
        *elem = res.to_string();
    }
}
