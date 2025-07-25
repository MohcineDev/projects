pub fn fibonacci(n: u32) -> u32 {
    let mut nums: Vec<u32> = Vec::new();
    let mut a = 0;
    let mut b = 1;

    nums.push(a);
    nums.push(b);

    for _ in 0..n {
        let n = a + b;
        nums.push(n);
        a = b;
        b = n;
    }
    // for elem in nums {
    //     println!("{}", elem);

    // }

    return nums[n as usize];
    // if n == 1 {
    //     return 1;
    // }
    // if n==0 {
    //     return 0;
    // }
    // return fibonacci(n-1)+ fibonacci(n-2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // let result = add(2, 2);
        // assert_eq!(result, 4);
    }
}
