fn main(){
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
        )
        .json()
        .init();
    process_order(101,2,true);
    process_order(102,0,true);
    process_order(103,5,false);
}
fn process_order(order_id:u32,quantity:u32,database_available:bool){
    tracing::info!(order_id,quantity,"processing order");
    if quantity == 0{
        tracing::warn!(
            order_id,
            quantity,
            "order rejected: quantity must be grater than zero"
        );
        return;
    }
    if !database_available{
        tracing::error!(
            order_id,
            error = "database_unavailable",
            "could not save order"
        );
        return;
    }
    tracing::info!(
        order_id,
        quantity,
        "order processed successfully"
    );
}
