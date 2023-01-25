fn main() {
    let mut s1: String = String::from("hello");
    change(&mut s1);

    println!("{}", s1);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// The rules of references
// 1. At any given time, you can have either one mutable reference of any number of immutable
//    references.
// 2. References must always be valid.
