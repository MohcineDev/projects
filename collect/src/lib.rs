pub fn bubble_sort(arr: &mut [i32]) {
    //let mut sorted_list = vec![];
    // sorted_list = arr.to_vec();

    println!("{:?}", arr);

    // let mut i = 0;
    // let mut j = 0;
    // arr.sort();

    // for (i, a) in sorted_list.iter().enumerate() {
    // for (i, _a) in arr.iter_mut().enumerate() {
    //     for (j, _b) in arr.iter_mut().enumerate() {
    //         if arr[i] > arr[j] {
    //             let temp = &arr[i];
    //             arr[i] = arr[j];
    //             arr[j] = *temp;
    //         }
    //     }
    // }

    // for i in 0..arr.len()  {
    //     for j in 0..arr.len() -1{
    //         if arr[j] > arr[j+1] {
    //             let temp = arr[j];
    //             arr[j] = arr[j+1];
    //             arr[j+1] = temp;
    //         }
    //     }
    // }

    for i in 0..arr.len()  {
        for j in i..arr.len(){
            if arr[i] > arr[j] {
                let temp = arr[i];
                arr[i] = arr[j];
                arr[j] = temp;
            }
        }
    }

}

//   println!("sorted_list : {:?}", sorted_list);
