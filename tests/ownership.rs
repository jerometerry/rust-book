#[test]
fn test_variable_bindings() {
    let v = vec![1, 2, 3];
}

#[test]
fn test_move_semantics() {
    let v = vec![1, 2, 3];

    let v2 = v;
}

// won't compile. v has moved. Vectors are not copy types
//#[test]
//fn test_move_semantics_no_compile() {
//    let v = vec![1, 2, 3];
//
//    let v2 = v;
//
//    println!("v[0] is: {}", v[0]);
//}

fn take(v: Vec<i32>) {
}

// won't compile. v has moved
//#[test]
//fn test_take() {
//    let v = vec![1, 2, 3];
//    
//    take(v);
//    
//    println!("v[0] is: {}", v[0]);
//}

#[test]
fn test_move_semantics_mutable() {
    let v = vec![1, 2, 3];

    let mut v2 = v;
}

// This works because integers are copy types
#[test]
fn test_copy_i32() {
    let v = 1;

    let v2 = v;

    println!("v is: {}", v);
}

#[test]
fn test_copy_double() {
    let a = 5;

    let _y = double(a);
    println!("{}", a);
}

fn double(x: i32) -> i32 {
    x * 2
}

fn test_copy_bool() {
    let a = true;

    let _y = change_truth(a);
    println!("{}", a);
}

fn change_truth(x: bool) -> bool {
    !x
}

fn hand_back_ownership(v: Vec<i32>) -> Vec<i32> {
    v
}

fn hand_back_ownership_mult(v1: Vec<i32>, v2: Vec<i32>) -> (Vec<i32>, Vec<i32>, i32) {
    (v1, v2, 42)
}

#[test]
fn test_hand_back_ownership() {
    let v1 = vec![1, 2, 3];
    let v2 = vec![1, 2, 3];

    let (v1, v2, awnser) = hand_back_ownership_mult(v1, v2);
}

