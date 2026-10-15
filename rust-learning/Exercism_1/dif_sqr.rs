fn square_of_sum(n:u32)->u32{
    let mut total = 0;
    for numbers in 1..=n{
        total = total + numbers;
    }
    total = total * total;
    total

}
fn sum_of_squares(n:u32)->u32{
    let mut square = 0; //5 
    for numbers in 1..=n{
        square = square + (numbers * numbers);
    }
    square

}
fn difference(n : u32)-> u32{

    let dif = square_of_sum(n) - sum_of_squares(n);
    dif

}
fn main(){
    println!("{}",square_of_sum(3));
    println!("{}",sum_of_squares(3));
    println!("{}",difference(3));
}