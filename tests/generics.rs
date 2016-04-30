// Option is defined in Rust's standard library
// enum Option<T> {
//      Some(T),
//      None,
// }

#[test]
fn test_generics() {
    let x: Option<i32> = Some(3);
}

// won't compile. Types don't match
//#[test]
//fn test_generics_fail() {
//    let x: Option<f64> = Some(5);
//}

#[test]
fn test_generics_2() {
    let x: Option<i32> = Some(5);
    let y: Option<f64> = Some(5.0f64);
}

// another type defined in Rust's standard library
// enum Result<T, E> {
//      Ok(T),
//      Err(E),
//  }

fn takes_anything<T>(x: T) {
}

#[test]
fn test_generics_3() {
    takes_anything("hello, world!");
    takes_anything(5);
    takes_anything(5.1234);
}

fn takes_two_of_the_same_thing<T>(x: T, y:T) {
}

#[test]
fn test_generics_4() {
    takes_two_of_the_same_thing(1, 2);
    takes_two_of_the_same_thing("hello", "world");
}

fn takes_two_types<T, U>(x: T, y: U) {
}

#[test]
fn test_generics_5() {
    takes_two_types(1, 5.0);
    takes_two_types(7, "hello");
}

struct Point<T> {
    x: T,
    y: T,
}

#[test]
fn test_generic_struct() {
    let int_origin = Point { x: 0, y: 0 };
    let float_origin = Point { x: 0.0, y: 0.0 };
}

impl<T> Point<T> {
    fn swap(&mut self) {
        std::mem::swap(&mut self.x, &mut self.y);
    }
}

