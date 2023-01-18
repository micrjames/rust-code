fn main() {
    let s1: String = gives_ownership();
    let s2: String = String::from("hello");
    let s3: String = takes_and_gives_back(s2);
    println!("s1 = {}, s3 = {}", s1, s3);
}

fn gives_ownership() -> String {
    let some_string: String = String::from("hello");
    some_string     // Moves ownership to s1
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
