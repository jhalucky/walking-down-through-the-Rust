fn main() {
    let s1 = String::from("hello"); 
    let s2 = s1; 
    println!("{}", s1); // This line will cause an error 
    // because s1 has been moved to s2, and ownership says data can only have one owner at a time
}