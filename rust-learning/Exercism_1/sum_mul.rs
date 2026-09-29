fn sum_multiples(limit:u32)->u32{   
    let mut positve_int = 0;
    for number in 1..limit{
        if number % 3 == 0|| number % 5 == 0{
            positve_int = positve_int + number;
        }
    }
    positve_int
}
fn main(){
    println!("{}",sum_multiples(16));
}