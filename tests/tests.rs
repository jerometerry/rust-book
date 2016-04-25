fn main() {
}

#[test]
fn test_shadowing() {
    let mut x: i32 = 1;
    x = 7;
    let x = x;

    let y = 4;
    let y = "I can also be bound to text!";

    print_number(5);
    print_number(add_one(6));

    let f: fn(i32) -> i32;
}

fn print_number(x: i32) {
    println!("x is: {}", x);
}

fn add_one(x: i32) -> i32 {
    x + 1
}

#[test]
fn test_if() {
    let x = 5;

    if x == 5 {
        println!("x is five!");
    } else if x == 6 {
        println!("x is six!");
    } else {
        println!("x is not five or six :(");
    }
}

#[test]
fn test_while() {
    let mut x = 5;
    let mut done = false;

    while !done {
        x += x - 3;

        println!("{}", x);

        if x % 5 == 0 {
            done = true;
        }
    }
}

#[test]
fn test_for() {
    for x in 0..10 {
        println!( "{}", x );
    }

}

#[test]
fn test_enumerate() {
    for (i,j) in (5..10).enumerate() {
        println!("i = {} and j = {}", i, j);
    }
}

#[test]
fn test_iteration() {
    let lines = "hello\nworld".lines();

    for (linenumber, line) in lines.enumerate() {
        println!("{}: {}", linenumber, line);
    }
}

#[test]
fn test_iteration_end_early() {
    let mut x = 5;
    let mut done = false;

    while !done {
        x += x - 3;

        println!("{}", x);

        if x % 5 == 0 {
            done = true;
        }
    }
}

#[test]
fn test_iteration_end_early_2() {
    let mut x = 5;

    loop {
        x += x - 3;

        println!("{}", x);

        if x % 5 == 0 { break; }
    }
}

#[test]
fn test_for_continue() {
    for x in 0..10 {
        if x % 2 == 0 { continue; }
    }
}

#[test]
fn test_loop_labels() {
    'outer: for x in 0..10 {
        'inner: for y in 0..10 {
            if x % 2 == 0 { continue 'outer; }
            if y % 2 == 0 {continue 'inner; }
            println!("x: {}, y:{}", x, y);
        }
    }
}
