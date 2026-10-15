fn delivery_fee(order_total: u32)-> u32{
    if order_total >= 100{
        return 0;
    }
    else if order_total >= 50 {
        return 5;
    }
    else{
        return 10;
    }
}

fn main(){
    let deliveryfee = delivery_fee(120);
    println!("{deliveryfee}");
}