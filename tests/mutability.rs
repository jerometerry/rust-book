use std::sync::Arc;
use std::cell::RefCell;
use std::cell::Cell;

// won't compile. x is immutable
//#[test]
//fn test_immutability() {
//    let x = 5;
//    x = 6; 
//}
//

#[test]
fn test_mutability() {
    let mut x = 5;
    x = 6;
}

#[test]
fn test_mutable_ref() {
    let mut x = 5;
    let y = &mut x;
}

#[test]
fn test_mutable_ref_2() {
    let mut x = 5;
    let mut y = &mut x;
}

#[test]
fn test_mut_pattern() {
    let (mut x, y) = (5, 6);
}

fn foo(mut x: i32) {
}

#[test]
fn test_exterior_mutability() {
    let x = Arc::new(5);
    let y = x.clone();
}

#[test]
#[should_panic]
fn test_double_borrow_expect_panic() {
    let x = RefCell::new(42);
    let y = x.borrow_mut();
    let z = x.borrow_mut();
}

struct Point {
    x: i32,
    y: i32,
}

// won't compile. b.x is immutable
//#[test]
//fn test_immutable_struct_field() {
//    let mut a = Point { x: 5, y: 6 };
//    a.x = 10;
//    let b = Point { x: 5, y: 6 };
//    b.x = 10;
//}
//

struct Point2 {
    x: i32,
    y: Cell<i32>,
}

#[test]
fn test_field_level_mutability() {
    let point = Point2 { x: 5, y: Cell::new(6) };
    point.y.set(7);
    println!("y: {:?}", point.y);
}


