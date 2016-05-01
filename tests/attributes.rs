#[test] // test annotation
fn test_1() {
}

#[should_panic] // attriute to indicate the test should panic
fn test_2() {
    #![test] // non-idomatic way of defining a test, but illustrates how ! works
    assert!(false);
}

#[inline(always)]
fn super_fast_fn() {}

