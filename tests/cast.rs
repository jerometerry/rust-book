use std::mem;

#[test]
fn test_safe_case() {
    let x: i32 = 5;
    let y = x as i64;
}

#[test]
fn test_numeric_casts() {
    let one = true as u8;
    let at_sign = 64 as char;
    let two_hundred = -56i8 as u8;
}

#[test]
fn test_cast_pointer() {
    let a = 300 as *const char;
    let b = a as u32;
}

// won't compile. non-scalar cast
//#[test]
//fn test_failing_cast() {
//    let a = [0u8, 0u8, 0u8, 0u8];
//    let b = a as u32;
//}

#[test]
fn test_transmute() {
    unsafe {
        let a = [0u8, 0u8, 0u8, 0u8];
        let b = mem::transmute::<[u8; 4], u32>(a);
    }
}

// won't complile. not same sizes
//#[test]
//fn test_transmute_incorrect_size() {
//    unsafe {
//        let a = [0u8, 0u8, 0u8, 0u8];
//        let b = mem::transmute::<[u8; 4], u64>(a);
//    }
//}

