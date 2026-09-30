fn delivery_fee(order_total : u32) -> u32{
    if order_total >= 100{
        0
    }
    else if order_total >= 50{
        5
    }
    else{
        10
    }
}
fn main(){
    println!("{}",delivery_fee(100))
}