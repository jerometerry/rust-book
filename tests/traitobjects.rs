trait Foo {
    fn method(&self) -> String;
}

impl Foo for u8 {
    fn method(&self) -> String { format!("u8: {}", *self) }
}

impl Foo for String {
    fn method(&self) -> String { format!("string: {}", *self) }
}

fn do_something<T: Foo>(x: T) {
    x.method();
}

#[test]
fn test_static_dispatch() {
    let x = 5u8;
    let y = "Hello".to_string();

    do_something(x);
    do_something(y);
}

fn do_something_via_trait_object(trait_object: &Foo) {
    trait_object.method();
}

#[test]
fn test_dynamic_dispatch() {
    let x = 5u8;
    do_something_via_trait_object(&x as &Foo);
}

#[test]
fn test_coerce() {
    let x = "Hello".to_string();
    do_something_via_trait_object(&x);
}

// won't compile. Clone is not object-safe
//#[test]
//fn test_invalid_trait_object() {
//    let v = vec![1, 2, 3];
//    let trait_object = &v as &Clone;
//}

