use raster::Color;
use rand::Rng
use std::f64::{self, consts};
 

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
            x: rng.gen_range(0..width),
            y: rng.gen_range(0..height),
        }
    }
}


/// triangle ::


 
 

 pub struct Triangle<'a> {
    p1: &'a Point,
    p2: &'a Point,
    p3: &'a Point,
}

impl<'a> Triangle<'a> {
     fn new(p1: &'a Point, p2: &'a Point, p3: &'a Point) -> Self {
        Triangle { p1, p2, p3 }
    }
}

////////////// line 

pub struct Line<'a> {
    start: &'a Point,
    end: &'a Point,
}

impl<'a> Line<'a> {
    fn new(start: &'a Point, end: &'a Point) -> Self {
        Line { start, end }
    }
        pub fn random(p1: &'a Point, p2: &'a Point) -> Self {
         Line { start: p1, end: p2 }
    }

    
}


////rectangle 

//  a new rectangle should be created from references to two different points. >>>>           # 
 
pub struct Rectangle<'a> {
    top_left: &'a Point,
    bottom_right: &'a Point,
}

impl<'a> Rectangle<'a> {
    fn new(top_left: &'a Point, bottom_right: &'a Point) -> Self {
        Rectangle { top_left, bottom_right }
    }
    
}


/////////////////////cercle 
pub struct Circle<'a> {
    center: &'a Point,
    radius: i32,
}

impl<'a> Circle<'a> {
    pub fn new(center: &'a Point, radius: i32) -> Self {
        Circle { center, radius }
    }

    pub fn random(center: &'a Point) -> Self {
        let mut rng = rand::thread_rng();
        let radius = rng.gen_range(1..=50);
        Circle { center, radius }
    }
}



//drawable


pub trait Drawable {
    fn draw<I: Displayable>(&self, image: &mut I);
    fn color(&self) -> Color {
        Color::white() 
    }
}


//Displayable
pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}



//drawable for point 


impl Drawable for Point {
    fn draw<I: Displayable>(&self, image: &mut I) {
         image.display(self.x, self.y, self.color());
    }

    fn color(&self) -> Color {
         Color::white()
    }
}
////drawable for rec

impl<'a> Drawable for Rectangle<'a> {
    fn draw<I: Displayable>(&self, image: &mut I) {
        let color = self.color();

         let top_left = self.top_left;
        let top_right = Point { x: self.bottom_right.x, y: self.top_left.y };
        let bottom_right = self.bottom_right;
        let bottom_left = Point { x: self.top_left.x, y: self.bottom_right.y };

         Line::new(top_left, &top_right).draw(image);
        Line::new(&top_right, bottom_right).draw(image);
        Line::new(bottom_right, &bottom_left).draw(image);
        Line::new(&bottom_left, top_left).draw(image);
    }

    fn color(&self) -> Color {
        Color::white() 
    }
}


///drawabl fror tri 

impl<'a> Drawable for Triangle<'a> {
    fn draw<I: Displayable>(&self, image: &mut I) {
         Line::new(self.p1, self.p2).draw(image);
        Line::new(self.p2, self.p3).draw(image);
        Line::new(self.p3, self.p1).draw(image);
    }

    fn color(&self) -> Color {
        Color::white()  
    }
}

//fraw the line Bresenham algo 
impl<'a> Drawable for Line<'a> {
    fn draw<I: Displayable>(&self, image: &mut I) {
        let x1 = self.start.x;
        let y1 = self.start.y;
        let x2 = self.end.x;
        let y2 = self.end.y;

        let color = self.color();

        let dx = (x2 - x1).abs();
        let dy = (y2 - y1).abs();

        let sx = if x1 < x2 { 1 } else { -1 };
        let sy = if y1 < y2 { 1 } else { -1 };

        let mut err = if dx > dy { dx / 2 } else { -(dy / 2) };

        let mut x = x1;
        let mut y = y1;

        loop {
            image.display(x, y, color);
            if x == x2 && y == y2 {
                break;
            }
            let e2 = err;
            if e2 > -dx {
                err -= dy;
                x += sx;
            }
            if e2 < dy {
                err += dx;
                y += sy;
            }
        }
    }

    fn color(&self) -> Color {
        Color::white()
    }
}
/////////drawable for circle


impl<'a> Drawable for Circle<'a> {
    fn draw<I: Displayable>(&self, image: &mut I) {
        let center_x = self.center.x;
        let center_y = self.center.y;
        let radius = self.radius;

        let color = self.color();

        let mut x = radius;
        let mut y = 0;
        let mut err = 0;

        while x >= y {
             image.display(center_x + x, center_y + y, color);
            image.display(center_x + y, center_y + x, color);
            image.display(center_x - y, center_y + x, color);
            image.display(center_x - x, center_y + y, color);
            image.display(center_x - x, center_y - y, color);
            image.display(center_x - y, center_y - x, color);
            image.display(center_x + y, center_y - x, color);
            image.display(center_x + x, center_y - y, color);

            y += 1;
            if err <= 0 {
                err += 2 * y + 1;
            } 
            if err > 0 {
                x -= 1;
                err -= 2 * x + 1;
            }
        }
    }

    fn color(&self) -> Color {
        Color::white()     
    }
}

///////


