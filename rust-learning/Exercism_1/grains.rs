fn square(a:u32)->u64{
    assert!(a >= 1 && a <= 64, "Square must be between 1 and 64");
    let mut grain =1;
    for _ in 1..a{
        grain = grain * 2;
       
    }
    grain
}
fn total()-> u64{
    let mut total = 0;
    for u in 1..=64{
        total += square(u);
    }
    total
}
fn main(){
    println!("{}",square(8));
    println!("{}",total());
}
