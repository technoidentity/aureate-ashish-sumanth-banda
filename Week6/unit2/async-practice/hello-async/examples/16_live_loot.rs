use std::time::Duration;

fn main() {
    trpl::block_on(async {
        let (sender, mut receiver) = trpl::channel();

        let boss = async move {
            for reward in ["Flaming Sword", "Dragon Shield", "Gold"] {
                trpl::sleep(Duration::from_secs(1)).await;
                sender.send(reward).unwrap();
            }
        };

        let inventory = async {
            while let Some(reward) = receiver.recv().await {
                println!("Collected: {reward}");
            }

            println!("All rewards collected!");
        };

        trpl::join(boss, inventory).await;
    });
}