#[test]
fn test_vec_create() {
    let v = vec![1, 2, 3, 4, 5];
}

#[test]
fn test_vec_repeating() {
    let v = vec![0; 10]; // ten zeros
}

#[test]
fn test_vec_access() {
    let v = vec![1, 2, 3, 4, 5];

    let i: usize = 0;
    let j: i32 = 0;

    // indexing must be done with usize
    v[i];

    // doesn't compile
    //v[j];
}

#[test]
#[should_panic]
fn test_out_of_bounds() {
    let v = vec![1, 2, 3];
    // should panic due to out of bounds
    println!("Item 7 is {}", v[7]);
}

#[test]
fn test_handle_out_of_bounds() {
    let v = vec![1, 2, 3];
    match v.get(7) {
        Some(x) => println!("Item 7 is {}", x),
        None => println!("Sorry, this vector is too short.")
    }
}

#[test]
fn test_iteration() {
    let mut v = vec![1, 2, 3, 4, 5];

    for i in &v {
        println!("A reference to {}", i);
    }

    for i in &mut v {
        println!("A mutable reference to {}", i);
    }

    for i in v {
        println!("Take ownership of the vector and its element {}", i);
    }
}

