use std::collections::HashMap;

pub fn mean(list: &[i32]) -> f64 {
    let sum: i32 = list.iter().sum();

    sum as f64 / list.len() as f64
}

pub fn median(list: &[i32]) -> i32 {
    let mut new_list: Vec<i32> = list.to_vec();
    let res;
    let len = list.len();
new_list.sort();

    //check if the list len is odd
    if new_list.len() % 2 == 1 {
        res = new_list[len / 2]
    } else {
        res = (new_list[len / 2] + new_list[len / 2 - 1]) / 2;
    }

    res
}

pub fn mode(list: &[i32]) -> i32 {
    let mut list_map: HashMap<i32, i32> = HashMap::new();
    let mut res=0;

    // fill the map
    for num in list {
        *list_map.entry(*num).or_insert(0) += 1;
    }

    let mut max: i32 = 0;
    for (k, v) in list_map {
        if v > max {
            max = v;
            res = k;
        }
    }

    res
}
