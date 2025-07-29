use std::collections::HashMap;

pub fn is_permutation(s1: &str, s2: &str) -> bool {
    let res: bool = true;
    let mut hash_one: HashMap<char, u8> = HashMap::new();
    let mut hash_sec: HashMap<char, u8> = HashMap::new();

    if s1.len() != s2.len() {
        return false;
    }

    //fill first map
    for c in s1.chars() {
        *hash_one.entry(c).or_insert(0) += 1;
    }

    //fill second map
    for c in s2.chars() {
        *hash_sec.entry(c).or_insert(0) += 1;
    }

    for (k, v) in hash_one.iter() {
        let count = hash_sec.get(k);
 

        if count == None|| count.unwrap() != v {
            return false;
        }
    }

    res
}
