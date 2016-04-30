use std::net::ToSocketAddrs;

#[test]
fn test_static_string_slice() {
    let greeting = "Hello there.";
}

#[test]
fn test_span_1() {
    let s = "foo
        bar";
    assert_eq!("foo\n        bar", s);
}

#[test]
fn test_span_2() {
    let s = "foo\
        bar";
    assert_eq!("foobar", s);
}

#[test]
fn test_mutable_string() {
    let mut s = "Hello".to_string();
    println!("{}", s);
    s.push_str(", world!");
    println!("{}", s);
}

fn takes_slice(slice: &str) {
    println!("Got: {}", slice);
}

#[test]
fn test_coerce() {
    let s = "Hello".to_string();
    takes_slice(&s);
}

fn connect<A: ToSocketAddrs>(addr: A) {
}

#[test]
fn test_trait_coerce() {
    let addr_string = "192.168.0.1:3000".to_string();
    connect(&*addr_string);
}

// won't compile
//#[test]
//fn test_indexing() {
//    let s = "hello";
//    println!("The first letter of s is {}", s[0]);
//}
//

#[test]
fn test_walk_bytes() {
    let hachiko = "忠犬ハチ公";

    for b in hachiko.as_bytes() {
            print!("{}, ", b);
    }

    println!("");

    for c in hachiko.chars() {
            print!("{}, ", c);
    }

    println!("");
}

#[test]
fn test_get_nth_byte() {
    let hachiko = "忠犬ハチ公";
    let dog = hachiko.chars().nth(1); 
}

#[test]
fn test_slicing_by_bytes() {
    let dog = "hachiko";
    let hachi = &dog[0..2];
}

#[test]
#[should_panic]
fn test_slicking_by_bytes_unicode() {
    let dog = "忠犬ハチ公";
    let hachi = &dog[0..2];
}

#[test]
fn test_concatenation_str() {
    let hello = "Hello ".to_string();
    let world = "world!";
    let hello_world = hello + world;
}

#[test]
fn test_concatenation_String() {
    let hello = "Hello ".to_string();
    let world = "world!".to_string();
    let hello_world = hello + &world;
}

