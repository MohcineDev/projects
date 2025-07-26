use rand::Rng;
use rand::distr::{Distribution, StandardUniform};
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
    println!("{}", ar[0]);

    let mut rng = rand::rng();

    // an unbiased integer over the entire range:
    let i: u8 = rng.random_range(0..3);
    println!("random i = {i}");
    let random_variant: Food = rand::random();
    println!("random i = {:?}", random_variant);
}
 #[derive(Debug)]
pub enum Food {
    Burger,
    Pizza,
    Kebab,
}

impl Distribution<Food> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Food {
        let index: u8 = rng.random_range(0..3);
        match index {
            0 => Food::Burger,
            1 => Food::Pizza,
            2 => Food::Kebab,
            _ => unreachable!(),
        }
    }
}
