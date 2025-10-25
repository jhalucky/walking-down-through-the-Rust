
fn main() {

    let mut count = 1;

    while count <= 20 {
        if count % 2 == 0 {
            println!("Even Count: {}", count);
        }
        count += 1;
    }
}