fn square_sum(n: u32) -> u32{
    let mut total = 0;
    for number in 1..=n{
        total = number + total;
    }
    total = total * total;
    total
}
fn sum_square(n: u32) -> u32{
    let mut total = 0;
    for number in 1..=n{
        total = total + (number * number);
    }
    total

}
fn diff(n: u32) -> u32{
    let first = square_sum(n);
    let second = sum_square(n);
    first - second
}

fn main(){
    println!("{}", diff(5));
}