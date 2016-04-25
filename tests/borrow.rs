#[test]
fn test_borrow() {
    let v1 = vec![1, 2, 3];
    let v2 = vec![1, 2, 3];

    let answer = foo(&v1, &v2);

    // we can use v1 and v2 here!
}

fn foo(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
    // do stuff with v1 and v2

    // hand back ownership, and the result of our function
    42
}

// won't compile, because v is immutable
//fn bar(v: &Vec<i32>) {
//    v.push(5);
//}
//

#[test]
fn test_mutable_references() {
    let mut x = 5;
    {
        let y = &mut x;
        *y += 1;
    }
    assert_eq!(x, 6);
}

// won't compile. because we’ve violated the rules: we have a &mut T pointing to x, 
// and so we aren’t allowed to create any &Ts. One or the other. 
//#[test]
//fn test() {
//    let mut x = 5;
//    let y = &mut x;
//
//    *y += 1;
//
//    println!("{}", x);
//}

#[test]
fn test_borrow_2() {
	let mut x = 5;
	{
		let y = &mut x;
		*y += 1;
	}

	println!("{}", x);
}

#[test]
fn test_borrow_loop() {
    let mut v = vec![1, 2, 3];

    for i in &v {
        println!("{}", i);
    }
}

// won't compile. Can't modify v becuase it's borrowed by the loop.
//#[test]
//fn test_borrow_loop_compile_error() {
//    let mut v = vec![1, 2, 3];
//
//    for i in &v {
//        println!("{}", i);
//        v.push(34);
//    }
//}

// won't compile. x doesn't live long enough
//#[test]
//fn test_use_after_free_failing() {
//    let y: &i32;
//    {
//        let x = 5;
//        y = &x;
//    }
//
//    println!("{}", x);
//}

// won't compile. reference is declared before variable it refers to.
// Resources within the same scope are freed in the opposite order they were declared
//#[test]
//fn test_user_after_free() {
//    let y: &i32;
//    let x = 5;
//    y = &x;
//
//    println!("{}", y);
//}

