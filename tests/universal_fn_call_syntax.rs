trait Foo {
    fn f(&self);
}

trait Bar {
    fn f(&self);
}

struct Baz;

impl Foo for Baz {
    fn f(&self) { println!("Baz's impl of Foo"); }
}

impl Bar for Baz {
    fn f(&self) { println!("Baz's impl of Bar"); }
}

// won't comple. multiple f() methods in scope
//#[test]
//fn test_call_duplicate_method() {
//    let b = Baz;
//    b.f();
//}

#[test]
fn test_disambiguate() {
    let b = Baz;
    Foo::f(&b);
    Bar::f(&b);
}

trait Foo1 {
    fn foo() -> i32;
}

struct Bar1;

impl Bar1 {
    fn foo() -> i32 {
        20 
    }
}

impl Foo1 for Bar1 {
    fn foo() -> i32 {
        10
    }
}

#[test]
fn test_disambiguate_expanded() {
    assert_eq!(10, <Bar1 as Foo1>::foo());
    assert_eq!(20, Bar1::foo());
}
