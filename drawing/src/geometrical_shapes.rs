use rand::Rng;
pub trait Drawable {
    fn draw(&self, image: &mut raster::Image);
    fn color(&self) -> raster::Color;
}
pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: raster::Color);
}

pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let mut rng = rand::thread_rng();
        Point {
            x: rng.gen_range(1..=width),
            y: rng.gen_range(1..=height),
            // y: rand::random::<i32>() % height,
        }
    }
}
pub struct Line {
    start: Point,
    end: Point,
}

impl Line {
    pub fn new(start: &Point, end: &Point) -> Self {
        Line {
            // start: Point::new(start.x, start.y),
            start: Point::new(start.x, start.y),
            end: Point::new(end.x, end.y),
        }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let start = Point::random(width, height);
        let end = Point::random(width, height);
        Line::new(&start, &end)
    }
}
pub struct Triangle {
    p1: Point,
    p2: Point,
    p3: Point,
}

impl Triangle {
    pub fn new(p1: &Point, p2: &Point, p3: &Point) -> Self {
        Triangle {
            p1: Point::new(p1.x, p1.y),
            p2: Point::new(p2.x, p2.y),
            p3: Point::new(p3.x, p3.y),
        }
    }
}
// pub struct Rectangle {
//     p1: Point,
//     p2: Point,
// }

// impl Rectangle {
//     pub fn new(p1: &Point, p2: &Point) -> Self {
//         Rectangle {
//             p1: Point::new(p1.x, p1.y),
//             p2: Point::new(p2.x, p2.y),
//         }
//     }
// }
// pub struct Circle {
//     center: Point,
//     radius: i32,
// }

// impl Circle {
//     pub fn new(center: &Point, radius: i32) -> Self {
//         Circle {
//             center: Point::new(center.x, center.y),
//             radius,
//         }
//     }

//     pub fn random(width: i32, height: i32) -> Self {
//         let center = Point::random(width, height);
//         let radius = rand::random::<i32>() % 100 + 10; // Random radius between 10 and 100
//         Circle::new(&center, radius)
//     }
// }
impl Drawable for Point {
    fn draw(&self, image: &mut raster::Image) {
        image.display(self.x, self.y, self.color());
    }

    fn color(&self) -> raster::Color {
        raster::Color::rgb(255, 0, 0)
    }
}

impl Drawable for Line {
    fn draw(&self, image: &mut raster::Image) {
        let dx = &self.end.x - &self.start.x;
        let dy = &self.end.y - &self.start.y;
        
        let steps;
        if dx.abs() > dy.abs() {
            steps = dx.abs();
        } else {
            steps = dy.abs();
        };
        let xinc = dx as f64 / steps as f64;
        let yinc = dy as f64 / steps as f64;

        let mut x = self.start.x as f64;
        let mut y = self.start.y as f64;

        let col = self.color();
        for _ in 0..=steps {
            // image.display(x, y, raster::Color::rgb(0, 255,0));
            image.display(x as i32, y as i32, col.clone());
            x += xinc;
            y += yinc;
        }
    }

    fn color(&self) -> raster::Color {
        let mut rng = rand::thread_rng();
        raster::Color::rgb(
            rng.gen_range(0..=255), // Random red value
            rng.gen_range(0..=255), // Random green value
            rng.gen_range(0..=255), // Random blue value
        )
    }
}
impl Drawable for Triangle {
    fn draw(&self, image: &mut raster::Image) {
        Line::new(&self.p1, &self.p2).draw(image);
        Line::new(&self.p2, &self.p3).draw(image);
        Line::new(&self.p3, &self.p1).draw(image);
    }

    fn color(&self) -> raster::Color {
        // raster::Color::rgb(0, 255, 0) // Green color for the triangle

        let mut rng = rand::thread_rng();
        raster::Color::rgb(
            rng.gen_range(0..=255), // Random red value
            rng.gen_range(0..=255), // Random green value
            rng.gen_range(0..=255), // Random blue value
        )
    }
}
// impl Drawable for Rectangle {
//     fn draw(&self, image: &mut raster::Image) {
//         // Draw the edges of the rectangle
//     }

//     fn color(&self) -> raster::Color {
//         raster::Color::rgb(255, 255, 0) // Yellow color for the rectangle
//     }
// }
// impl Drawable for Circle {
//     fn draw(&self, image: &mut raster::Image) {
//         // Circle drawing algorithm
//     }

//     fn color(&self) -> raster::Color {
//         raster::Color::rgb(0, 255, 255) // Cyan color for the circle
//     }
// }
