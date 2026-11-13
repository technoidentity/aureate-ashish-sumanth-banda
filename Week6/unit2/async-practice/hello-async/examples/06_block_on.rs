async fn get_price()-> u32{
    println!("2.Calculating price");
    100
}
fn main(){
    println!("1. Before block_on");
    let price = trpl::block_on(async{
        let result = get_price().await;
        result

    });
    println!("3.price:{price}");
}