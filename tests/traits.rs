use std::fmt::Debug;

struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}

impl HasArea for i32 {
    fn area(&self) -> f64 {
        println!("this is silly");
        *self as f64
    }
}

// won't compile. area is not defined for T
//fn print_area<T>(shape: T) {
//    println!("This shape has an area of {}", shape.area());
//}

fn print_area<T: HasArea>(shape: T) {
    println!("This shape as an area of {}", shape.area());
}

struct Square {
    x: f64,
    y: f64,
    side: f64,
}

impl HasArea for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

#[test]
fn test_traits() {
    let c = Circle {
        x: 0.0f64,
        y: 0.0f64,
        radius: 1.0f64,
    };

    let s = Square {
        x: 0.0f64,
        y: 0.0f64,
        side: 1.0f64,
    };

    print_area(c);
    print_area(s);
}

struct Rectangle<T> {
    x: T,
    y: T,
    width: T,
    height: T,
}

impl<T: PartialEq> Rectangle<T> {
    fn is_square(&self) -> bool {
        self.width == self.height
    }
}

#[test]
fn test_traits_2() {
    let mut r = Rectangle {
        x: 0,
        y: 0,
        width: 47,
        height: 47,
    };

    assert!(r.is_square());

    r.height = 42;
    assert!(!r.is_square());
}

#[test]
fn test_area_i32() {
    println!("{}", 5.area());
}

// won't compile. Need 'use std::io::Write;' to get access to .write()
// this prevents methods from being added to types unless explicity using the traits.
//#[test]
//fn test_traits_missig() {
//    let mut f = std::fs::File::open("foo.txt").expect("Couldn’t open foo.txt");
//    let buf = b"whatever"; // byte string literal. buf: &[u8; 8]
//    let result = f.write(buf);
//}

 fn foo<T: Clone + Debug>(x: T) {
    x.clone();
    println!("{:?}", x);
 }

#[test]
fn test_multiple_trait_bounds() {
    foo(5);
}

fn foo2<T: Clone, K: Clone + Debug>(x: T, y: K) {
    x.clone();
    y.clone();
    println!("{:?}", y);
}

#[test]
fn test_multi_arg_with_traits() {
    foo2(5, "hello");
}

fn bar<T, K>(x: T, y: K) 
    where T: Clone, 
          K: Clone + Debug {
    x.clone();
    y.clone();
    println!("{:?}", y);
}

#[test]
fn test_trait_where_clause() {
    bar(5, "hello");
}

trait ConvertTo<Output> {
    fn convert(&self) -> Output;
}

impl ConvertTo<i64> for i32 {
    fn convert(&self) -> i64 { *self as i64 }
}

// can be called with T == i32
fn normal<T: ConvertTo<i64>>(x: &T) -> i64 {
    x.convert()
}

// can be called with T == i64
fn inverse<T>() -> T
        // this is using ConvertTo as if it were "ConvertTo<i64>"
        where i32: ConvertTo<T> {
    42.convert()
}

trait Foo {
    fn is_valid(&self) -> bool;
    fn is_invalid(&self) -> bool { !self.is_valid() }
}

struct UseDefault;

impl Foo for UseDefault {
    fn is_valid(&self) -> bool {
        println!("Called CuseDefault.is_valid");
        true
    }
}

struct OverrideDefault;

impl Foo for OverrideDefault {
    fn is_valid(&self) -> bool {
        println!("Called OverrideDefault.is_valid");
        true
    }

    fn is_invalid(&self) -> bool {
        println!("Called OverrideDefault.is_invalid!");
        true
    }
}

#[test]
fn test_trait_override() {
    let default = UseDefault;
    assert!(!default.is_invalid());

    let over = OverrideDefault;
    assert!(over.is_invalid());
}

trait Foo1 {
    fn foo(&self);
}

trait FooBar : Foo1 {
    fn foobar(&self);
}

struct Baz;

impl Foo1 for Baz {
    fn foo(&self) { println!("foo"); }
}

impl FooBar for Baz {
    fn foobar(&self) { println!("foobar"); }
}

#[derive(Debug)]
struct Foo2;

#[test]
fn test_derived_trait() {
    println!("{:?}", Foo2);
}
