fn main(){
    let test = String :: from ("Rust is fun");
    let word = &test[0..4];
    println!("{}",word);
    println!("{}",test);
}