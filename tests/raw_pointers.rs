#[test]
fn test() {
    let x = 5;
    let raw = &x as *const i32;

    let mut y = 10;
    let raw_mut = &mut y as *mut i32;
}

// won't compile. use of raw pointer is unsafe
//#[test]
//fn test_2() {
//    let x = 5;
//    let raw = &x as *const i32;
//    println!("raw points at {}", *raw);
//}

#[test]
fn test_3() {
    let x = 5;
    let raw = &x as *const i32;
    let points_at = unsafe { *raw };
    println!("raw points at {}", points_at);
}

#[test]
fn test_4() {
    // explicit cast
    let i: u32 = 1;
    let p_imm: *const u32 = &i as *const u32;

    // implicit coercion
    let mut m: u32 = 2;
    let p_mut: *mut u32 = &mut m;

    // The &*x dereferencing sytle is preferred to using a transmute
    unsafe {
        let ref_imm: &u32 = &*p_imm;
        let ref_mut: &mut u32 = &mut *p_mut;
    }
}

