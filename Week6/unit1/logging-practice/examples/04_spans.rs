fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
        )
        .json()
        .init();

    process_order("req-001", 101, 2, true);
    process_order("req-002", 102, 0, true);
    process_order("req-003", 103, 5, false);
}

fn process_order(
    request_id: &str,
    order_id: u32,
    quantity: u32,
    database_available: bool,
) {
    let span = tracing::info_span!(
        "process_order",
        request_id,
        order_id,
        quantity
    );

    let _guard = span.enter();

    tracing::info!("Processing started");

    if quantity == 0 {
        tracing::warn!("Order rejected: quantity must be greater than zero");
        return;
    }

    if !database_available {
        tracing::error!(
            error = "database_unavailable",
            "Could not save order"
        );
        return;
    }

    tracing::info!("Order processed successfully");
}