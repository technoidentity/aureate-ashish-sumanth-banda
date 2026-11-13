fn main(){
    tracing_subscriber::fmt::init();
    tracing::info!("Logging practice started");
    tracing::warn!("order quantity is unusually high");
    tracing::error!("could not save order: database unavailable");
}