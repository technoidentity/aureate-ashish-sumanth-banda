fn raindrops(a : u32)->String{
    let mut sounds = String::new();
    if a % 3 == 0{  
        sounds.push_str("Pling");
    }
    if a % 5 == 0{
        sounds.push_str("Plang");
    } 
    if a % 7 == 0{
        sounds.push_str("Plong");
    }
    if sounds.is_empty(){
        return a.to_string();
    }
    sounds
}
fn main(){
    let a = 4;
    println!("{}",raindrops(a));
}