fn main() {
    println!("{}", reverse_it(123));
    println!("{}", reverse_it(-123));
}

pub fn reverse_it(v: i32) -> String {
    let mut res = String::from("");
    let mut num = v;
    let mut sign = false;

    if v < 0 {
        num = v * -1;
        sign = true;
    }

    let mut ss = num.to_string();
    let mut o: String = ss.chars().rev().collect();
    if sign {
        res.push_str("-");
    }
    res.push_str(&o);
    res.push_str(&ss);
    res
}

/*
321123
-321123
*/
