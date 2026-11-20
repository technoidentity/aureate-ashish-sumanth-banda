async fn prepare_order(){
    println!("2.preparing order");
}
fn main(){
    trpl::block_on(async{
        println!("1.Before creating the future");
        let order = prepare_order();
        println!("3.Future created");
        order.await;
        println!("4.order finished");
    }
    );
}