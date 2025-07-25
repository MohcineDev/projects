#[derive(Debug,PartialEq)]
pub struct Matrix(pub (i32, i32), pub (i32, i32));

pub fn transpose(m: Matrix) -> Matrix {
    let tup = m.0;
    let tup1 = m.1;

    Matrix((tup.0, tup1.0), (tup.1, tup1.1))

    // Matrix((m.0.0, m.1.0), (m.0.1, m.0.1))
}
/*
1 3
4 5
*/