fn main() {
    let s: String = String::from("hello");
    takes_ownership(some_string: s);

    println!("{}", s);

    let x: i32 = 5;
    makes_copy(some_integer: x);
    println!("{}", x);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}
fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
