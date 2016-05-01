macro_rules! myvec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

#[test]
fn test_custom_macro() {
    let v = myvec![1,2,3];
}
