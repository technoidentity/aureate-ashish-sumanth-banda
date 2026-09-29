fn main(){
    let mut count = 0;
    let mut total = 0;
    for number in 1..=10{
        if number % 2 == 0{
            count = count+1;
            total = total + number;
        }
    }
    println!("Count: {count}");
    println!("Total: {total}");
}