use std::ops::Add;
use std::ops::Mul;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point { x: self.x + other.x, y: self.y + other.y }
    }
}

#[test]
fn test_plus_operator_overload() {
    let p1 = Point { x: 1, y: 0 };
    let p2 = Point { x: 2, y: 3 };
    let p3 = p1 + p2;
    println!("{:?}", p3);
}

impl Add<i32> for Point {
    type Output = Point;

    fn add(self, rhs: i32) -> Point {
        Point { x: self.x + rhs, y: self.y + rhs }
    }
}

#[test]
fn test_add_i32_to_point() {
    let p1 = Point { x: 0, y: 0 };
    let p2 = p1 + 10;
}

trait HasArea<T> {
    fn area(&self) -> T;
}

struct Square<T> {
    x: T,
    y: T,
    side: T,
}

// T must implement Mul so we can multiply the two sides
// T must implement Copy so Rust doesn't try to move self.side into the return value
impl<T> HasArea<T> for Square<T>
        where T: Mul<Output=T> + Copy {
    fn area(&self) -> T {
        self.side * self.side
    }
}

#[test]
fn test_operator_traits_in_generic_structs() {
    let s = Square {
        x: 0f64,
        y: 0f64,
        side: 12.0f64,
    };
    println!("Area of s: {}", s.area());
}

