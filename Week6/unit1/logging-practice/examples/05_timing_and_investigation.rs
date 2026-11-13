use std::time::{Duration, Instant};
fn main(){
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
        )
        .json()
        .with_writer(std::io::stdout)
        .init();
    process_order("req-001",101,2,true);
    process_order("req-002",102,0,true);
    process_order("req-003",103,5,false);
    fn process_order(
        request_id:&str,
        order_id:u32,
        quantity:u32,
        database_available:bool,

    ){
        let span = tracing::info_span!(
            "process_order",
            request_id,
            order_id,
            quantity
        );
        let _guard = span.enter();
        let start = Instant::now();
        tracing::info!("Processing started");
        tracing::debug!(database_available, "checking order conditions");
        tracing::trace!("entering validation");

        let result: Result<(),&str >= if quantity == 0{
            Err("invalid_quantity")
        }
        else if !database_available{
            Err("database_unavailable")
        }
        else{
            std::thread::sleep(Duration::from_millis(50));
            Ok(())
        };
        let duration_ms = start.elapsed().as_secs_f64()*1000.0;
        match result{
            Ok(())=>{
                tracing::info!{
                    status ="success",
                    duration_ms,
                    "order process successfully"
                };

            }
            Err("invalid_quantity")=>{
                tracing::warn!(
                    status = "rejected",
                    error = "invalid_quantity",
                    duration_ms,
                    "quantity must be grater than zero"
                );

            }
            Err(reason)=>{
                tracing::error!(
                    status = "failed",
                    error = reason,
                    duration_ms,
                    "could not save order"
                );
            }
        }
    }
}