fn previous_number(number: u32)->Option<u32>{
    if number > 0{
        let number = number-1;
        Some(number)
    }
    else{
        None
    }
}
fn main(){
    let number = 0;
    let result = previous_number(number);
    if let Some(prev) = result{
        println!("{prev}");
    }
    
    // match result{
    //     Some(prev) => println!("The previous number is {}",{prev}),
    //     None => println!("Nothing"),
    // }
}