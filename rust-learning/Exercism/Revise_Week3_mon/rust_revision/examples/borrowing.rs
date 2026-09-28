fn main() {
    let name = String::from("Ashish");
    greet(&name);
    println!("{}",name);
}
fn greet(name: &str){
    println!("Hello,{}",name);
}