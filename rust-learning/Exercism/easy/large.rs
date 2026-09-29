fn larger(a: u32, b: u32) -> u32{
    if a > b {
        a
    }
    else if b > a{
        b
    }
    else {
        a
    }
}
fn main(){
    println!("{}", larger(5,5));
}