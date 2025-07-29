use std::f64::consts::{PI};

#[derive(Debug, Clone, Copy)]
pub struct Circle {
    pub center: Point,
    pub radius: f64,
}

impl Circle {
    pub fn new(a: f64, b: f64, c: f64) -> Self {
        Circle {
            center: Point(a, b),
            radius: c,
        }
    }
    pub fn area(self) -> f64 {
        let res: f64;
        res = PI * self.radius * self.radius;
        res
    }
    pub fn diameter(self) -> f64 {
        let res: f64;
        res = self.radius * 2.0;
        res
    }
    pub fn intersect(self, c: Circle) -> bool {
        let centers_dis = self.center.distance(c.center);
        let mut res = false;

        if centers_dis < self.diameter() || centers_dis < c.diameter() {
            res = true;
        }

        res
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Point(pub f64, pub f64);

impl Point {
    pub fn distance(self, p: Point) -> f64 {
        let  res: f64;
        // √((x2 - x1)² + (y2 - y1)²)
        /*
        1 3
        4 5
         */
        let left: f64 = (p.0 - self.0) * (p.0 - self.0);
        let right: f64 = (p.1 - self.1) * (p.1 - self.1);

        res = (left + right).sqrt();
        res
    }
}
