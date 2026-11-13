use std::fmt::Debug;
fn compare<T>(a:&T, b:&T)
where
    T:PartialOrd + Debug,
{
    println!("{:?} > {:?}: {}", a, b, a>b);
}

fn show(value: &impl Debug){
    println!("{:?}",value);
}
fn make_number()-> impl Debug{
    25
}
fn main(){
    compare(&10)
}