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

#[test]
fn test_move() {
    let num = 5;
    let owns_num = move |x: i32| x + num;
}

#[test]
fn test_move_mutable_borrow() {
    let mut num = 5;
    {
        // closure gets a mutable borrow of num
        let mut add_num = |x: i32| num += x;
        add_num(5);
    }
    assert_eq!(10, num);
}

#[test]
fn test_move_closure() {
    let mut num = 5;
    {
        // closure gets a copy of num
        // moving a closure effective gives the closure it's own stack frame
        let mut add_num = move |x: i32| num += x;
         add_num(5);
    }
    assert_eq!(5, num);
}

// closure is statically dispatched due to the Fn trait
fn call_with_one_static_dispatch<F>(some_closure: F) -> i32
    where F : Fn(i32) -> i32 {
        some_closure(1)
}

#[test]
fn test_pass_closure_as_arg_static_dispatch() {
    let answer = call_with_one_static_dispatch(|x| x + 2);
    assert_eq!(3, answer);
}

fn call_with_one_dynamic_dispatch(some_closure: &Fn(i32) -> i32) -> i32 {
    some_closure(1)
}

#[test]
fn test_pass_closure_as_arg_dynamic_dispatch() {
    let answer = call_with_one_dynamic_dispatch(&|x| x + 2);
    assert_eq!(3, answer);
}

fn add_one(i: i32) -> i32 {
    i + 1
}

#[test]
fn test_function_instead_of_closure() {
    let f = add_one;
    let answer = call_with_one_dynamic_dispatch(&f);
    assert_eq!(2, answer);
}

#[test]
fn test_functioin_instead_of_closure_2() {
    let answer = call_with_one_dynamic_dispatch(&add_one);
    assert_eq!(2, answer);
}

// won't compile. In order to return, Rust must know it's size.
// Fn is a trait, so it could be any size, hence the error
//fn factory() -> (Fn(i32) -> i32) {
//    let num = 5;
//    |x| x + num
//}

// won't compile. No lifetime specified for returned reference
//fn factory() -> &(Fn(i32) -> i32) {
//    let num = 5;
//    |x| x + num
//}

// won't compile. Mismatched types
// return value is actually [closure@closures.rs:151:5: 151:16 num:_] 
//fn factory() -> &'static (Fn(i32) -> i32) {
//    let num = 5;
//    |x| x + num
//}

// won't compile. 
// closure may outlive the current function, but it borrows `num`,
// which is owned by the current function
//fn factory() -> Box<Fn(i32) -> i32> {
//    let num = 5;
//    Box::new(|x| x + num)
//}

// By making the inner closure a move Fn, we create a new stack frame for
// our closure. By Boxing it up, we've given it a known size, and allowing it
// to escape out stack frame
fn factory() -> Box<Fn(i32) -> i32> {
    let num = 5;
    Box::new(move |x| x + num)
}

#[test]
fn test_function_returning_function() {
    let f = factory();
    let answer = f(1);
    assert_eq!(6, answer);
}

