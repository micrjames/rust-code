fn main() {
    let s1: String = String::from("hello");
    // let (s2, len) = calculate_length(s1);
    let len: usize = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
}

/*
fn calculate_length(s: String) -> (String, usize) {
    let length: usize = s.len();
    (s, length)
}
*/
fn calculate_length(s: &String) -> usize {
    let length: usize = s.len();
    length
}
