use std::collections::HashMap;
//Create a function named bigger that gets the biggest positive number in the HashMap.

pub fn bigger(mapp: HashMap<&str, i32>) -> i32 {
    let mut res: i32 = 0;

    for (_, value)in mapp.iter() {
        
            if *value > res {
                res = *value
            }
            
       
    }

    res
}
