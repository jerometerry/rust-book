#[test]
fn test_struct_2d_members() {
    let origin_x = 0;
    let origin_y = 0;
}

struct Point {
    x: i32,
    y: i32,
}

#[test]
fn test_immutable_struct() {
    let origin = Point { x: 0, y: 0 };
    println!("the origin is at ({}, {})", origin.x, origin.y);
}

#[test]
fn test_mutable_struct() {
    let mut point = Point { x: 0, y: 0 };
    point.x = 5;
    println!("The point is at ({},{})", point.x, point.y);
}

// won't compile. Mutability isn't supported on fields
//struct Point2 {
//    mut x: i32,
//    y: i32,
//}

// won't compile. point.y is immutable
//#[test]
//fn test_temp_mutability() {
//    let mut point = Point { x: 0, y: 0 };
//    point.x = 5;
//    let point = point;
//    point.y = 6;
//}

struct PointRef<'a> {
    x: &'a mut i32,
    y: &'a mut i32,
}

#[test]
fn test_struct_mut_pointers() {
    let mut point = Point { x: 0, y: 0 };
    {
        let r = PointRef { x: &mut point.x, y: &mut point.y };
        *r.x = 5;
        *r.y = 6;
    }

    assert_eq!(5, point.x);
    assert_eq!(6, point.y);
}

struct Point3d {
    x: i32,
    y: i32,
    z: i32,
}

#[test]
fn test_struct_copy() {
    let mut point = Point3d { x: 0, y: 0, z: 0 };
    point = Point3d { y: 1, .. point };
    assert_eq!(0, point.x);
    assert_eq!(1, point.y);
    assert_eq!(0, point.z);
}

#[test]
fn test_struct_copy_2() {
    let origin = Point3d { x: 0, y: 0, z: 0 };
    let point = Point3d { z: 1, x: 2, .. origin };
    assert_eq!(2, point.x);
    assert_eq!(0, point.y);
    assert_eq!(1, point.z);
}

struct ColorTuple(i32, i32, i32);
struct PointTuple(i32, i32, i32);

#[test]
fn test_struct_tuple() {
    let black = ColorTuple(0, 0, 0);
    let origin = PointTuple(0, 0, 0);
}

// structs are almost always better than tuple structs
// struct values can be referenced by name.
// tuple struct values are referenced by positions
struct Color {
    red: i32,
    blue: i32,
    green: i32,
}

struct Inches(i32);

#[test]
fn test_newtype() {
    let length = Inches(10);
    let Inches(integer_length) = length;
    println!("lehgth is {} inches", integer_length);
}

struct Electron;

#[test]
fn test_unit_like_structs() {
    let x = Electron;
}
