fn is_leap(year: u32) ->bool{
    if year%400 == 0{
        true
    }
    else if year % 100 == 0 {
        false
    }
    else if year % 4 == 0{
        true
    }
    else{
        false
    }
}
fn main(){
    let year = 2024;
    println!("{}",is_leap(year));
}