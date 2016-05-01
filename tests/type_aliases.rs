use std::result;

type Name = String;

#[test]
fn test() {
    let x: Name = "Hello".to_string();
}

// won't compile. Can't compare types that don;t match
//#[test]
//fn test_2() {
//    let x: i32 = 5;
//    let y: i64 = 5;
//
//    if x == y {
//    }
//}

type Num = i32;

// this test passes, because Num is the same type as i32,
// not a new type
#[test]
fn test_3() {
    let x: i32 = 5;
    let y: Num = 5;

    if x == y {
    }
}

enum ConcreteError {
    Foo,
    Bar,
}

// Commonly used way in standard libary to create custom errors for each subsection
type Result<T> = result::Result<T, ConcreteError>;
