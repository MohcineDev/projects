pub fn is_empty(v: &str) -> bool {
    if v.len() == 0 {
        return true;
    }
    return false;
}

pub fn is_ascii(v: &str) -> bool {
    v.is_ascii()
}

pub fn contains(v: &str, pat: &str) -> bool {
    v.contains(pat)
}

pub fn split_at(v: &str, index: usize) -> (&str, &str) {
    //     let len = v.len();

    //     let  s1 = String::from("");
    //     let  s2 = String::from("");

    //     println!("{}", len);
    //     println!("{}", s1);
    //     println!("{}", s2);
    // let a = &s1;
    // let b = &s2;
    //     (a, b)
    v.split_at(index)
}

pub fn find(v: &str, pat: char) -> usize {
    v.find(pat).expect("something wrong")
}
