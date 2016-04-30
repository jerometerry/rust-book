struct HasDrop;

impl Drop for HasDrop {
    fn drop(&mut self) {
        println!("Dropping!");
    }
}

#[test]
fn test_drop() {
    let x = HasDrop;
} // drop will be called here, whex x goes out of scope

struct Firework {
    strength: i32,
}

impl Drop for Firework {
    fn drop(&mut self) {
        println!("BOOM times{}!!!", self.strength);
    }
}

#[test]
fn test_multi_drop() {
    let firecracker = Firework { strength: 1 };
    let tnt = Firework { strength: 100 };
} // tnt is dropped before firecracker. Drop is LIFO
