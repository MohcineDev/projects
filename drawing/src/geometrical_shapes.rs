use rand::Rng;
use raster::Color;
pub trait Drawable {
    fn draw(&self, image: &mut raster::Image);
    fn color(&self) -> raster::Color;
}
pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: raster::Color);
}
#[derive(Debug, Clone, Copy)]
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
#[derive(Debug, Clone, Copy)]
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
            // start: *start,
            // end: *end,
        }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let start = Point::random(width, height);
        let end = Point::random(width, height);
        Line::new(&start, &end)
    }
}

impl Drawable for Point {
    fn draw(&self, image: &mut raster::Image) {
        image.display(self.x, self.y, self.color());
    }

    fn color(&self) -> raster::Color {
        raster::Color::rgb(255, 0, 0)
    }
}

//https://www.geeksforgeeks.org/computer-graphics/dda-line-generation-algorithm-computer-graphics/
//DDA (Digital Differential Analyzer)
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

        for _ in 0..=steps {
            image.display(x as i32, y as i32, self.color());
            x += xinc;
            y += yinc;
        }
    }

    fn color(&self) -> raster::Color {
        raster::Color::rgb(255,255,255)
    }
}

//
// Triangle logic
//
#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    p1: Point,
    p2: Point,
    p3: Point,
}

impl Triangle {
    pub fn new(p1: &Point, p2: &Point, p3: &Point) -> Self {
        Triangle {
            p1: Point { x: p1.x, y: p1.y },
            p2: Point { x: p2.x, y: p2.y },
            p3: Point { x: p3.x, y: p3.y },
        }
    }
}

impl Drawable for Triangle {
    fn draw(&self, image: &mut raster::Image) {
        Line::new(&self.p1, &self.p2).draw(image);
        Line::new(&self.p2, &self.p3).draw(image);
        Line::new(&self.p3, &self.p1).draw(image);
    }

    fn color(&self) -> raster::Color {
        raster::Color::rgb(0, 255, 0)
    }
}
//
//Rectangle logic
//
#[derive(Debug, Clone, Copy)]
pub struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    pub fn new(p1: &Point, p2: &Point) -> Self {
        Rectangle {
            p1: Point::new(p1.x, p1.y),
            p2: Point::new(p2.x, p2.y),
        }
    }

    // add random rec
    pub fn random(width: i32, height: i32) -> Self {
        let p1 = Point::random(width, height);
        let p2 = Point::random(width, height);
        Rectangle::new(&p1, &p2)
    }
}

impl Drawable for Rectangle {
    fn draw(&self, image: &mut raster::Image) {
        //set corners
        //top left point
        let p1 = &self.p1;
        //bottom right point
        let p2 = &self.p2;
        //top right point
        let p3 = &Point::new(p2.x, p1.y);
        //bottom left point
        let p4 = &Point::new(p1.x, p2.y);

        Line::new(p1, p3).draw(image);
        Line::new(p3, p2).draw(image);
        Line::new(p2, p4).draw(image);
        Line::new(p4, p1).draw(image);
    }

    fn color(&self) -> raster::Color {
        raster::Color::rgb(255, 255, 0)
    }
}

//
// circle logic
//
pub struct Circle {
    center: Point,
    radius: i32,
}

impl Circle {
    pub fn new(center: &Point, radius: i32) -> Self {
        Circle {
            center: Point::new(center.x, center.y),
            radius,
        }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let center = Point::random(width, height);
        let radius = rand::random::<i32>() % 100 + 10;
        Circle::new(&center, radius)
    }
}

//https://www.youtube.com/watch?v=hpiILbMkF9w
impl Drawable for Circle {
    fn draw(&self, image: &mut raster::Image) {

        let cx = self.center.x;
        let cy = self.center.y;
        let r = self.radius;

        let mut x = 0;
        let mut y = -r;
        let mut p = -r;
  
            let col= self.color();

        while x < -y {
            if p > 0 {
                y += 1;
                p += 2 * (x + y) + 1;
            } else {
                p += 2 * x + 1;
            }


            image.display(cx + x, cy + y, col.clone());
            image.display(cx - x, cy + y, col.clone());
            image.display(cx + x, cy - y, col.clone());
            image.display(cx - x, cy - y, col.clone());
            image.display(cx + y, cy + x, col.clone());
            image.display(cx + y, cy - x, col.clone());
            image.display(cx - y, cy + x, col.clone());
            image.display(cx - y, cy - x, col.clone());
            x += 1;
        }
    }

    fn color(&self) -> raster::Color {
        // raster::Color::rgb(0, 255, 255)
              let mut rng = rand::thread_rng();

           raster::Color::rgb(
            rng.gen_range(0..=255),
            rng.gen_range(0..=255),
            rng.gen_range(0..=255),
        )
    }
}