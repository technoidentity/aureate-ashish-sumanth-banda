fn main() {
    trpl::block_on(async {
        let (tx, mut rx) = trpl::channel();

        let message = String::from("Order ready");

        tx.send(message).unwrap();

        let received = rx.recv().await.unwrap();

        println!("Waiter received: {received}");
    });
}