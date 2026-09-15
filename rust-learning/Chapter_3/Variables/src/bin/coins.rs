const COIN_BONUS: u32 = 5; 
fn main(){
    let mut coins = 10;
    println! ("The value of coin : {coins}");
    coins = coins + COIN_BONUS;
    println!("The value of coin : {coins}");
}