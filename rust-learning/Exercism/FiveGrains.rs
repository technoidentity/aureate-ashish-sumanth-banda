pub fn square(s: u32) -> u64 {
    assert!(s >= 1 && s <= 64, "Square must be between 1 and 64");

    let mut grains = 1;
    for _ in 1..s {
        grains *= 2;
    }
    grains
}

pub fn total() -> u64 {
    let mut total = 0;
    for s in 1..=64 {
        total += square(s);
    }
    total
}
fn main(){
    println!("{}", total());
}