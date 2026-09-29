fn safe_subtract(a: u32,b:u32)->Option<u32>{
    if a >= b {
        Some(a-b)
    }
    else{
        None
    }
}
fn main(){
    let a = 9;
    let b = 6;
    let s = safe_subtract(a,b);
    match s{
        Some(value)=>println!("{value}"),
        None =>println!("Cannot subtract"),
    }
}