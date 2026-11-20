use tokio::time::{sleep, Duration};

async fn process_order(order_id: u32) {
    println!("starting order {order_id}");
    sleep(Duration::from_secs(2)).await;
    println!("finished order {order_id}");
}

#[tokio::main]
async fn main() {
    let first = tokio::spawn(process_order(1));
    let second = tokio::spawn(process_order(2));

    first.await.expect("order 1 task failed");
    second.await.expect("order 2 task failed");

    println!("all orders finished");
}