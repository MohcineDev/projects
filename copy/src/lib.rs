pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    let f = c as f64;
    let tup = (c, f.exp(), f.abs().ln());
    tup
}

pub fn str_function(a: String) -> (String, String) {
    // println!("{}",a);
    let b = a.split_whitespace();

    let mut last_str = String::from("");
    for elem in b {
        let as_nbr: f64 = elem.parse().expect("can't convert");
        last_str.push_str(as_nbr.exp().to_string().trim());
        last_str.push_str(" ");
    }

    let tup = (a.clone(), last_str.trim().to_string());
    tup
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    // pub fn vec_function(b: Vec<i32>) ->u32{
 
    let mut f_nbrs = vec![];

    for elem in b.clone() {
        let f = elem as f64;

        f_nbrs.push(f.abs().ln());
    }

    let tup = (b, f_nbrs);

    tup
}
