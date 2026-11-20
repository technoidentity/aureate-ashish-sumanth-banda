fn main(){
    trpl::block_on(async{
        let handle = trpl::spawn_task(async{
            trpl::sleep(
                std::time::Duration::from_secs(2)
            ).await;
            println!("order ready");
        });
        println!("checking payment");
        handle.await.unwrap();
        println!("everything finished");
    });
}