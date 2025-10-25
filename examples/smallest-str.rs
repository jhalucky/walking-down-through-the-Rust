fn find_smallest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1.len() < str2.len() {
        str1
    } else {
        str2
    }
}

fn main() {
    let string1 = String::from("apple");
    let string2 = String::from("banana");

    let smallest = find_smallest(&string1, &string2);
    println!("The smallest string is: {}", smallest);
}