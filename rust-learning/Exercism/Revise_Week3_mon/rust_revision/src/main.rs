fn main() {

    println!("Whats your age ?");
    let mut input: String = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Could not read input"); 
    let age: u32 = input.trim().parse().expect("Enter a valid whole number") ;
    println!("Hi, your age is {}", age);
}
