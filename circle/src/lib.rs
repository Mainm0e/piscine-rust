// todo :  circle
/*
Instructions
Create the structures Circle and Point. You'll need to create the necessary methods for the code in the usage to compile, and give the expected output.

Methods:
Point:
distance() -> returns the distance between two coordinates.
Circle:
diameter() -> returns the diameter of the circle.
area() -> returns the area of the circle.
intersect() -> which returns true, if 2 circles intersect.
Associated Functions
Circle:
new() -> receives three 64bit floating point numbers in the following order: x, y and radius (x and y are the coordinates of the center of the new circle). The function returns a new circle.
Expected Functions and Structures
This snippets are incomplete, you'll need to complete them. You'll find some useful information in the usage.

#[derive(Debug)]
pub struct Circle {
	pub center //..
	pub radius //..
}

impl Circle {
    // ...
}

#[derive(Debug)]
pub struct Point {
    // ...
}

impl Point {
    // ...
}
 */

#[derive(Debug)]
pub struct Circle {
    pub center: Point,
    pub radius: f64,
}
impl Circle{
    pub fn new(x: f64, y: f64, radius: f64) -> Circle {
        Circle {
            center: Point { x, y },
            radius,
        }
    }
    pub fn diameter(&self) -> f64 {
        self.radius * 2.0
    }
    pub fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius.powi(2)
    }
    pub fn intersect(&self, other: &Circle) -> bool {
        self.center.distance(&other.center) < self.radius + other.radius
    }


}

#[derive(Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn distance(&self, other: &Point) -> f64 {
        f64::sqrt((self.x - other.x).powi(2) + (self.y - other.y).powi(2))
    }
}


