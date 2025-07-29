use std::collections::HashMap;

pub fn word_frequency_counter<'a>(words: &[&'a str]) -> HashMap<&'a str, usize> {
    let mut a: HashMap<&str, usize> = HashMap::new();
    for word in words {
        if let Some(&w) = a.get(word) {
            a.insert(word, w + 1);
        } else {
            a.insert(&word, 1);
        }
    }
    a
}

pub fn nb_distinct_words(frequency_count: &HashMap<&str, usize>) -> usize {
   // let   a: HashMap<&str, usize> = HashMap::new();

    // for word in frequency_count {
    //     a.insert(word, 1);
    // }
    frequency_count.len()
}
