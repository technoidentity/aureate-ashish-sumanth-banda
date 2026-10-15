fn main(){
    let coins = 17;
    let players = 5;
    let share = coins/players;
    let leftover = coins % players;
    let divides_evenly = leftover==0;
    let lives: u16 = 300;
    let temperature: i8 = -10;
    let mut counter: u16 = 255;
    counter = counter + 1;
    let price: f32 =12.5;
    let speed =2.5;
    let badge = 'R';
    let item = ("pen",3,true);
    let count = item.1;
    let (name,quantity,available) =item;
    let marks =[50,60,70];
    let first =marks[0];
    let split = (share,leftover);
    let remaining = split.1;

    let total = (marks[0]+marks[1]+marks[2]);
    println!("Total ={total}");
    
    println! ("sp = {remaining}");
    
    println!("{share}");
    println!("{leftover}");
    println!("The value of the divide {divides_evenly}");
    println!("{lives}");
    println!("{temperature}");
    println!("{counter}");
    println!("{name}");
    println!("{quantity}");
    println!("{available}");
}