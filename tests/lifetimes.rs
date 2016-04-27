// implicit lifetime
fn foo(x: &i32) {
}

// explicit lifetime
fn bar<'a>(x: &'a i32) {
}

fn bar_mut<'a>(x: &'a mut i32) {
}

struct Foo<'a> {
    x: &'a i32,
}

#[test]
fn test_strut_lifetime() {
    let y = &5;
    let f = Foo { x: y };
    println!("{}", f.x);
}

impl<'a> Foo<'a> {
    fn x(&self) -> &'a i32 { self.x }
}

#[test]
fn test_struct_function() {
    let y = &5;
    let f = Foo { x: y };

    println!("x is: {}", f.x());
}

// multiple references, same lifetime
fn x_or_y_1<'a>(x: &'a str, y: &'a str) -> &'a str {
    x
}

// multiple references, different lifetimes
fn x_or_y_2<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    x
}

// will not compile. &5 and &f.x don't live long enough
//#[test]
//fn test_invalid_scope() {
//    let x;
//    {
//        let y = &5;
//        let f = Foo { x: y };
//        x = &f.x;
//    }
//
//    println!("{}", x);
//}
//

// 'static lifetime has length of entire program
#[test]
fn test_static_lifetime() {
    let x: &'static str = "Hello, world!";
}

static FOO: i32 = 5;
#[test]
fn test_static_globals() {
    let x: &'static i32 = &FOO;
}

// input lifetime
fn foo_2<'a>(bar: &'a str) {
}

// output lifetime
fn foo_3<'a>() -> &'a str {
   let x: &'static str = "Hello, world";
   x
}

// lifetime in both positions
fn foo_4<'a>(bar: &'a str) -> &'a str {
    bar
}
