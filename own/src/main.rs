fn main() {
    let x: i32 = 5;
    let y: i32 = x; // Copy
                    
    let s1: String = String::from("hello");
//    let s2: String = s1;    // Move (not shallow copy)
    let s2: String = s1.clone();

    println!("{}, world!", s2);
}
