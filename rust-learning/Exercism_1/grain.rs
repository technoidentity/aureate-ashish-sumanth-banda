fn square(s: u32)->u64{
    assert!(s>=1 && s<=64);
    let mut grain = 1;
    for numbers in 1..s{
        grain = grain * 2;
    }
    grain

}

fn total()->u64{
    let mut total = 0;
    for numbers in 1..=64{
        total = square(numbers)+total;
    }
    total
}
fn main(){
    println!("{}",total());
}