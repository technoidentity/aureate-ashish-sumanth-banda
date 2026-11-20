fn larger<'a,T>(first:&'a T, second:&'a T)->&'a T
where
    T:PartialOrd,
{
    if first > second{
        first
    }else{
        second
    }
}
fn main(){
    let topic: &'static str = "Rust practice";
    println!("Topic:{}",topic);
    let a = 10;
    let b = 25;
    println!("Larger number:{}",larger(&a,&b));

    let first_letter = 'a';
    let second_letter = 'z';
    println!("Larger letter : {}", larger(&first_letter,&second_letter));
}