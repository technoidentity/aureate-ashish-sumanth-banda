fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()

        )
        .json()
        .init();
    tracing::info!(
        order_id = 101,
        quantity = 2,
        "order processed"
    );
    tracing::warn!(
        order_id = 102,
        quantity = 1000,
        "order quantity is unusully high"
    );
    tracing::error!(
        order_id = 103,
        error = "database_unavaible",
        "could not save order"
    );
}
