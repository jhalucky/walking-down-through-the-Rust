fn main() {
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).expect("Failed to read line");
    let n: u32 = n.trim().parse().expect("Please enter a valid number");
    let mut factorial = 1;

    for i in 1..=n {
        factorial *= i;
    }

    println!("Factorial of {} is {}", n, factorial);
}