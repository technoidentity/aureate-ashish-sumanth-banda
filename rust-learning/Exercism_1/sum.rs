fn sum_to(n: u32)-> u32{
    let mut total = 0;
    for number in 1..=n{
        total = number+ total;
        //println!("{}",total);
    }
    return total
}
fn main(){
    println!("{}",sum_to(5))
}