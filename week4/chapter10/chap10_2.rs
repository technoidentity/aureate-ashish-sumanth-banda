fn main(){
    let number: Option<i32> = None;
    let letter: Option<char> = Some('z');
    let missing: Option<i32> = None;

    println!("number: {:?}",number);
    println!("letter: {:?}",letter);
    println!("missing: {:?}", missing);

    match number{
        Some(value)=> println!("Stored number: {}", value),
        None => println!("No number availble"),
    }
    let result: Result<i32, &str> = Ok(50);
    match result{
        Ok(value)=>println!("Success:{}",value),
        Err(message)=>println!("Error:{}",message),
    }
}
