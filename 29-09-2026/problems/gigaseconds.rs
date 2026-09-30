fn collatz(number:u64)->Option<u64>{
    //number = 4;
    let mut current = number;
    let mut steps = 0; 
    if number == 0{
        return None;
    }
    while current != 1{
        if current % 2 == 0{
            current = current / 2;
            steps = steps + 1; 
            
        }
        else{
            current = 1 + (current * 3);
            steps = steps + 1; 
        }
        
    }
    Some(steps)
}
fn main(){
    let num = 4;
    let result = collatz(num);
    match result {
        Some(item)=>println!("{}",item),
        None => println!("its invalid"),
    }
}
