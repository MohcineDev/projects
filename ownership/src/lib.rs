pub fn first_subword(s: String) -> String {
    let mut word = String::from("");

    //split the string mn inedx 1
    let (first_letter, rest) = s.split_at(1);

    // check if the rest contains an uppercase letter then split it with it
    if rest.contains(char::is_uppercase) {

        let second: Vec<&str> = rest.split(char::is_uppercase).collect();
        word.push_str(first_letter);
        word.push_str(second[0]);

    } else if s.contains('_') {
        let first: Vec<&str> = s.split('_').collect();
        word.push_str(first[0]);
    }else{
        word.push_str(&s);
    }
 

    word
}
