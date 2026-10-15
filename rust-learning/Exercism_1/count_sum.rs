fn count(n : [i32;5])->(u32,i32){
    let mut count = 0;
    let mut total = 0;
    for number in n{
        if number % 2 == 0{
            count = count+1;
            total = number + total;
        }
    }
    (count,total)
}
fn main(){
    let numbers = [4, 7, 2, 9, 6];
    let (even_count,even_total) =count(numbers);
    println!("{even_count}");
    println!("{even_total}");
}