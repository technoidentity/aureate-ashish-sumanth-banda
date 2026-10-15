fn can_enter(has_ticket: bool, has_pass: bool)-> bool{
    if has_pass || has_ticket{
        true
    }
    else{
        false
    }
}
fn main(){
    let entry = can_enter(false,false);
    println!("can enter? = {entry}");
}