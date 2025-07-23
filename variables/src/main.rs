fn main() {
    /*    let mut x = 5;
    println!("the value of x is: {x}");

    x = 6;
    println!("the value of x is: {x}");
     */

    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("the value of x in the inner scope is : {x}");
    }
    println!("the value of x is: {x}");

    let a = 2.5; //f64
    let b: f32 = 3.0561516654; //f32

    println!("{a} {}", b);

    //compound Types
    //Tuple type

    let tup: (i32, f64, u8) = (600, 6.4, 1);

    let (a, b, c) = tup;
    println!("{a} , {b} , {c}");

    //access by index
    let c = tup.2;
    println!("{c}");

    //Array
    let ar: [i32; 3] = [654, 46, 45];
}
