mod areas_volumes;

pub fn area_fit(
    //lenght - width
    (x, y): (usize, usize),
    // which shape
    kind: areas_volumes::GeometricalShapes,
    //number
    times: usize,
    //
    (a, b): (usize, usize),
) -> bool {
    /*
    a: represents the:

    side in case of a Square
    base in case of a Triangle
    radius in case of a Circle
    side_a in case of a Rectangle

b: represents the:

    height in case of a Triangle
    side_b in case of a Rectangle


     */
    todo!()
}
//xUvJ)j!F"a]]_86
pub fn volume_fit(
    (x, y, z): (usize, usize, usize),
    kind: areas_volumes::GeometricalVolumes,
    times: usize,
    (a, b, c): (usize, usize, usize),
) -> bool {
    todo!()
}
