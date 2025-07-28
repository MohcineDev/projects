use std::collections::HashMap;

pub fn is_permutation(s1: &str, s2: &str) -> bool {
    let mut s_hash: HashMap<char, u8> = HashMap::new();
    let mut res: bool = false;

    for c in s1.chars() {
        
        for (key, value) in s_hash.iter() {
            if *key == c {
                s_hash.insert(c, value + 1);
            } else {
                s_hash.insert(c, 1);
            }
        }
    }
    res
}
