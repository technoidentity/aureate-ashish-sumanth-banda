fn add(n: u32) -> u32{
    let mut total = 0; 
    for number in 1..=n{
        total = total + number;
    }
    total
}
fn main(){
    println!("{}",add(5));
}