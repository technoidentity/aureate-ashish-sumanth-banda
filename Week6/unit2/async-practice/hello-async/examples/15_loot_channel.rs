fn main(){
    trpl::block_on(async{
        let (loot_sender, mut inventory_receiver)=trpl::channel();
        loot_sender.send("flaming sward").unwrap();
        println!("boss defeated! reward send.");
        let reward = inventory_receiver.recv().await;
        println!("inventory received: {reward:?}");
    });
}