fn foo<T>(x: T) {
}

fn bar() {
}

#[test]
fn test_using_match() {
    let option = Some(5);

    match option {
        Some(x) => { foo(x) },
        None => {}
    }
}

#[test]
fn test_using_if() {
    let option = Some(5);

    if option.is_some() {
        let x = option.unwrap();
        foo(x);
    }
}

#[test]
fn test_using_iflet() {
    let option = Some(5);

    if let Some(x) = option {
        foo(x);
    } else {
        bar();
    }
}

#[test]
fn test_whilelet() {
    let mut v = vec![1, 3, 5, 7, 11];
}
