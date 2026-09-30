fn square_of_sum(n:u32)->u32{
    let mut a = 0;
    for number in 1..=n{
        a = number + a;      
    }
    a = a*a;
    a
}
fn sum_of_squares(n:u32)->u32{
    let mut m = 0;
    for number in 1..=n{
        m = m + (number * number);
    }
    m

}
fn difference(n:u32)->u32{
    let dif = square_of_sum(n)-sum_of_squares(n);
    dif
}
fn main(){
    println!("{}",square_of_sum(3));
    println!("{}",sum_of_squares(3));
    println!("{}",difference(3));
}