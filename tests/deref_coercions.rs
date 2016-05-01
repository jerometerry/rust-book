use std::ops::Deref;
use std::rc::Rc;

struct DerefExample<T> {
    value: T,
}

impl<T> Deref for DerefExample<T> {
    type Target = T;

    fn deref(&self) -> &T {
         &self.value
    }
}

#[test]
fn test_deref() {
    let x = DerefExample { value: 'a' };
    assert_eq!('a', *x);
}

fn foo(s: &str) {
    // borrow a string for a second
}

#[test]
fn test_deref_coercion() {
    let owned = "Hello".to_string();
    foo(&owned);
}

#[test]
fn test_rc_deref_coercion() {
    let owned = "hello".to_string();
    let counted = Rc::new(owned);
    foo(&counted);
}

fn foo2(s: &[i32]) {
    // borrow a slice for a second
}

#[test]
fn test_deref_coercion_to_slice() {
    let owned = vec![1, 2, 3];
    foo2(&owned);
}

struct Foo;

impl Foo {
    fn foo(&self) { println!("Foo"); }
}

#[test]
fn test() {
    let f = &&&&&&&&&&&&&&&&&&&Foo;
    f.foo();
}
