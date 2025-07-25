pub fn factorial(num: u64) -> u64 {
    // let mut n: u64 = 1;

    if num == 0 {
        return 1;
    }
 
    return num * factorial(num-1);
}
