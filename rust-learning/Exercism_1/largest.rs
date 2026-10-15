fn largest(a:u32,b:u32,c:u32)->u32{
    if a >= b && a >= c{
        a
    }
    else if b >= a && b >= c{
        b
    }
    else{
        c
    }

}
fn main(){
    println!("{}",largest(4,4,2));
}