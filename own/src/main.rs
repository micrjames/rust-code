fn main() {
    fn a() {
        let x: &str = "hello";
        let y: i32 = 22;
        b();
    }

    fn b() {
        let x: String = String::from("world");
    }
}

// each value in Rust has a variable that's called its owner.
// there can only be one owner at a time.
// when the owner goes out of scope, the value will be dropped.
