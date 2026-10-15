fn half_if_even(number: u32) -> Option<u32>{
    if number % 2 == 0{
        let number = number /2;
        Some(number)
    }
    else{
        None
    }
}
fn main(){
    let number = 8;
    let result = half_if_even(number);
    match result{
        Some(half) => println!("Half is : {half}"),
        None => println!("The number is odd"),
    }
}