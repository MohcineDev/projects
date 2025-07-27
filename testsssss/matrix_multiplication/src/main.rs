#[derive(Debug)]
pub struct Matrix(pub(i32, i32),pub (i32, i32));

fn main() {
    let matrix = Matrix((1, 3), (4, 5));
    println!("Original matrix {:?}", matrix);
    println!("Matrix after multiply {:?}", multiply(matrix, 3));
}

pub fn multiply(m: Matrix, multiplier: i32) -> Matrix {

    let first = (m.0.0 * multiplier, m.0.1 * multiplier);
    let sec = (m.1.0 * multiplier, m.1.1 * multiplier);
    Matrix(first, sec)
}

/*
Original matrix Matrix ((1, 3), (4, 5))
Matrix after multiply ((3, 9), (12, 15))


pub struct Matrix(pub (i32, i32), pub (i32, i32));

pub fn transpose(m: Matrix) -> Matrix {
    let tup = m.0;
    let tup1 = m.1;

    Matrix((tup.0, tup1.0), (tup.1, tup1.1))

    // Matrix((m.0.0, m.1.0), (m.0.1, m.0.1))
}

*/
