fn main() {
    println!("Minimum and maximum are: {:?}", min_and_max(9, 2, 4));
}
pub fn min_and_max(nb_1: i32, nb_2: i32, nb_3: i32) -> (i32, i32) {
    let mut min = nb_1;
    let mut max = nb_1;

    if nb_2 < min {
        min = nb_2
    }
    if nb_3 < min {
        min = nb_3
    }
    if nb_2 > max {
        max = nb_2
    }
    if nb_3 > max {
        max = nb_3
    }

    (min, max)
}
