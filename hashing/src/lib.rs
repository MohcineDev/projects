use std::collections::HashMap;

pub fn mean(list: &[i32]) -> f64 {
    let sum: i32 = list.iter().sum();
    sum as f64 / list.iter().count() as f64
}

pub fn median(list: &[i32]) -> i32 {
    let res: i32;
    let mut arr_sort: Vec<i32> = vec![];

    for elem in list {
        arr_sort.push(*elem);
    }

    arr_sort.sort();
    println!("{:?}", arr_sort);

    if arr_sort.len() % 2 != 0 {
        res = arr_sort[arr_sort.len() / 2];
    } else {
        let half_len = arr_sort.len() / 2;
        let s = arr_sort[half_len] + arr_sort[half_len - 1];

        res = s / 2;
    }

    res
}

pub fn mode(list: &[i32]) -> i32 {
    let mut nums: HashMap<i32, i32> = HashMap::new();

    for a in list {
        *nums.entry(*a).or_insert(0) += 1;
    }

    let mut max = 0;
    let mut more_often: i32 = 0;

    for (k, v) in nums {
        if v > max {
            more_often = k;
            max = v;
        }
    }
    more_often
}
