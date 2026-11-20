use tokio::time::{sleep,Duration};
async fn process_order(order_id:u32){
    println!("Starting order {order_id}");
    sleep(Duration::from_secs(2)).await;
    println!("Finished order {order_id}");
}
#[tokio::main]
async fn main(){
    let _order = process_order(1);
    println!("main finished");
}