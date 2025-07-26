pub fn arrange_phrase(phrase: &str) -> String {
    println!("{}", phrase);
    let mut res: String = String::new();
    let mut words: Vec<&str> = phrase.split(" ").collect();
    let mut nums: Vec<i32> = vec![];

   println!("{:?}", words);
    for elem in &words {

        for c in elem.chars() {

            if c.is_numeric() {
                nums.push(c.to_string().parse().expect("can't convert!!"));
            }
        }

        // println!("{}", elem.contains(char::is_numeric));
    }
    println!("{:?}", nums);

    /////sort nums and words same time
    for i in 0..nums.len() {
       // println!("{} {}", i, nums[i]);

        for j in 0..nums.len() - i - 1 {
            if nums[j] > nums[j + 1] {
                let temp = nums[j + 1];
                nums[j + 1] = nums[j];
                nums[j] = temp;

                let word_temp = words[j + 1];
                words[j + 1] = words[j];
                words[j] = word_temp;
            }
        }
    }

    println!("sort :  - - -- {:?}", nums);
    println!("{:?}", words);

    let new_str: String = String::from(words.join(" "));
    for elem in new_str.chars(){
        if !elem.is_numeric() {
            res.push(elem);
        }
    }
    res
}
