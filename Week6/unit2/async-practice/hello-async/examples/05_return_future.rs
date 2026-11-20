use std::future::Future;
fn greeting(name:String)->impl Future<Output = String> {
    async move{
        format!("Hello, {name}")
    }
}
fn main(){
    trpl::block_on(async{
        let name = String::from("Ashish");
        let future = greeting(name);
        println!("Future created");
        let message = future.await;
        println!("{message}");
    });
}