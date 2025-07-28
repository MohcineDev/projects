use std::collections::HashMap;

fn main() {
    let mut hash = HashMap::new();
    // hash.insert("Cat", 122);
    // hash.insert("Dog", 333);
    // hash.insert("Elephant", 65);
    // hash.insert("Gorilla", 14);

    println!(
        "The smallest of the elements in the HashMap is {}",
        smallest(hash)
    );
}

pub fn smallest(h: HashMap<&str, i32>) -> i32 {
    let mut max = i32::MAX;
    if h.len() == 0 {
        return max;
    }
    for (_, d) in h.iter() {
        if *d < max {
            max = *d;
        }
    }

    return max;
}
