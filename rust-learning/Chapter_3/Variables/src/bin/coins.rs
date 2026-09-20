const COIN_BONUS: u32 = 5; 
fn main(){
    let mut coins = 10;
    println! ("The value of coin : {coins}");
    coins = coins + COIN_BONUS;
    {
        coins = coins * 2;
        println! ("Inside = {coins}");
    }
    println!("The value of coin outside : {coins}");
    let label = "coins";
    let label = label.len();
    println! ("Length: {label}");
}