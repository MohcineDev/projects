use std::collections::HashMap;

pub fn is_permutation(s1: &str, s2: &str) -> bool {
    let mut s_hash: HashMap<char, u8> = HashMap::new();
    let mut second_hash: HashMap<char, u8> = HashMap::new();
    let mut res: bool = true;
    if s1.len() != s2.len() {
        return false;
    }

    for c in s1.chars() {
        *s_hash.entry(c).or_insert(0) += 1;
    }
    for c in s2.chars() {
        *second_hash.entry(c).or_insert(0) += 1;
    }
    println!("{:?}", s_hash);
    println!("{:?}", second_hash);

    if s_hash.len() != second_hash.len() {
        return false;
    }
    for (key, value) in s_hash.iter() {
        if let Some(count) = second_hash.get(key) {
            if value != count {
                res = false;
            }
        }else{
            res = false;
        }
    }
    res
}
