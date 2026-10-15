fn main(){
    let numbers = [3, 8, 5, 2, 7];
    let mut count = 0;
    let mut total = 0;
    for number in numbers{
        if number % 2 != 0{
            count = count+1;
            total = number + total;
        }
    }
    println!("{count} ");
    println!("{total}");
}