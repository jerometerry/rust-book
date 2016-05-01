#[test]
fn test_basic_closure() {
    let plus_one = |x: i32| x + 1;
    assert_eq!(2, plus_one(1));
}

#[test]
fn test_longer_closure() {
    let plus_two = |x| {
        let mut result: i32 = x;
        result += 1;
        result += 1;
        result
    };
    assert_eq!(4, plus_two(2));
}

#[test]
fn test_basic_closure_with_annotations() {
    let plus_one = |x: i32| -> i32 { x + 1 };
    assert_eq!(2, plus_one(1));
}

fn  plus_one_v1   (x: i32) -> i32 { x + 1 }

#[test]
fn test_closure_syntax() {
    let plus_one_v2 = |x: i32| -> i32 { x + 1 };
    let plus_one_v3 = |x: i32|          x + 1  ;
}

#[test]
fn test_binding_scope() {
    let num = 5;
    let plus_num = |x: i32| x + num;
    assert_eq!(10, plus_num(5));
}

// won't compile. can't take a mutable borrown on num 
// becuase the closure is already borrowing it
//#[test]
//fn test_failing_borrow() {
//    let mut num = 5;
//    let plus_num = |x: i32| x + num;
//    let y = &mut num;
//}

#[test]
fn test_borrow() {
    let mut num = 5;
    {
        let plus_num = |x: i32| x + num;
    } 
    let y = &mut num; // borrow is valid here becuase closure is out of scope
}

// won't compile. ownership was passed to the closure, the same as if nums 
// was passed to a function call
//#[test]
//fn test_failing_move() {
//    let nums = vec![1, 2, 3];
//    let takes_nums = || nums;
//    println!("{:?}", nums);
//}
