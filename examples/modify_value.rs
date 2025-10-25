fn modify_value(value: &mut i32) {
    *value += 10;
}

fn main() {
    let mut num = 5;
    println!("Before modification: {}", num);
    modify_value(&mut num);
    println!("After modification: {}", num);
}