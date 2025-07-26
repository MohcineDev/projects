pub fn to_url(s: &str) -> String {
    let mut a = String::from("");

    for elem in s.chars() {
        
        if elem == ' ' {
            a.push_str("%20");
        } else{
            a.push(elem);
        }
        
    }
    a
}
