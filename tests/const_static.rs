#[test]
fn teest_const() {
    const N: i32 = 5;
}

#[test]
fn test_static() {
    static N: i32 = 5;
    static NAME: &'static str = "Steve";
}

#[test]
fn test_mutable_static() {
    static mut N: i32 = 5;

    // mutating static variables is unsafe
    unsafe {
        N += 1;
        println!("N: {}", N);
    }
}

