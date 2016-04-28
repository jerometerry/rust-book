fn func_patterns(x: i32)  -> &'static str {
    match x {
        1 => "one",
        2 => "two",
        3 => "three",
        _ => "anything",
    }
}

#[test]
fn test_patterns() {
    let r = func_patterns(1);
    assert_eq!("one", r);
}

#[test]
fn test_shadown() {
    let x = 1;
    let c = 'c';

    match c {
        // x shadows the outer scoped x value of 1
        // prints 'x: c c: c'
        x => println!("x: {} c: {}", x, c),
    };

    // prints '1'
    println!("x: {}", x);
}

#[test]
fn test_multiple_patterns() {
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

struct Point {
    x: i32,
    y: i32,
}

#[test]
fn test_destructuring() {
    let origin = Point { x: 0, y: 0 };

    match origin {
        Point { x, y } => println!("({}, {})", x, y),
    }
}

#[test]
fn test_destructuring_rename() {
    let origin = Point { x: 0, y: 0 };

    match origin {
        Point { x: x1, y: y1 } => println!("({}, {})", x1, y1),
    }
}

#[test]
fn test_destructuring_partial() {
    let origin = Point { x: 0, y: 0 };

    match origin {
        Point { y, .. } => println!("y is {}", y),
    }
}

// you can use _ in a pattern to disregard the type and value
//match some_value {
//    Ok(value) => println!("got a vlue: {}", value),
//    Err(_) -> println!("an error occurred"), 
//}
//

fn coordinate() -> (i32, i32, i32) {
    (0, 0, 0)
}

#[test]
fn test_destructure_triple_partial() {
    let (x, _, z) = coordinate();
}

// won't compile. use of partially moved value: 'tuple'
//#[test]
//fn test_bind_moving() {
//    let tuple: (u32, String) = (5, String::from("five"));
//    let (x, _s) = tuple;
//    println!("Tuple is: {:?}", tuple);
//}

#[test]
fn test_underscore_bind_not_moving() {
    let tuple = (5, String::from("five"));
    let (x, _) = tuple;
    println!("Tuple is: {:?}", tuple);
}

#[test]
fn test_drop_immediate() {
    let _ = String::from("    hello    ").trim();
}

enum OptionalTuple {
    Value(i32, i32, i32),
    Missing,
}

#[test]
fn test_blah() {
    let x = OptionalTuple::Value(5, -2, 3);

    match x {
        OptionalTuple::Value(..) => println!("Got a tuple!"),
        OptionalTuple::Missing => println!("No such luck."),
    }
}

#[test]
fn test_ref() {
    let x = 5;

    match x {
        ref r => println!("Got a reference to {}", r),
    }
}

#[test]
fn test_mut_ref() {
    let mut x = 5;

    match x {
        ref mut mr => println!("Got a mutable reference to {}", mr),
    }
}

#[test]
fn test_ranges() {
    let x = 1;

    match x {
        1 ... 5 => println!("one through five"),
        _ => println!("anything"),
    }
}

#[test]
fn test_special_chars() {
    let x = '💅';
    match x {
        'a' ... 'j' => println!("early letter"),
        'k' ... 'z' => println!("late letter"),
        _ => println!("something else"),
    }
}

#[test]
fn test_bind() {
    let x = 1;

    match x {
        e @ 1 ... 5 => println!("got a range element {}", e),
        _ => println!("anything"),
    }
}

#[derive(Debug)]
struct Person {
    name: Option<String>,
}

#[test]
fn test_match_data_structure() {
    let name = "Steve".to_string();
    let x: Option<Person> = Some(Person { name: Some(name) });
    match x {
        Some(Person { name: ref a @ Some(_), .. }) => println!("{:?}", a),
        _ => {}
    }
}

#[test]
fn test_bind_2() {
    let x = 5;

    match x {
        e @ 1 ... 5 | e @ 8 ... 18 => println!("got a range element {}", e),
        _ => println!("anything"),
    }
}

enum OptionalInt {
    Value(i32),
    Missing,
}

#[test]
fn test_guard() {
    let x = OptionalInt::Value(5);

    match x {
        OptionalInt::Value(i) if i > 5 => println!("Got an int bigger than five!"),
        OptionalInt::Value(..) => println!("Got an int!"),
        OptionalInt::Missing => println!("No such luck."),
    }
}


#[test]
fn test_guard_2() {
    let x = 4;
    let y = false;

    match x {
        4 | 5 if y => println!("yes"),
        _ => println!("no"),
    }
}

// mix and match
//match x {
//    Foo { x: Some(ref name), y: None } => ...
//}
