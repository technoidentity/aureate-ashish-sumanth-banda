fn count_multiples(n:u32)->u32{
    let mut count = 0;
    for number in 1..=n{
        if number % 3 ==0{
            count = count+1;
            //println!("{}",count);
        }
    }
    return count
}
fn main(){
    println!("{}",count_multiples(20))
}