fn reverse(Word:&str)-> String{
    let mut g = String::new();
    for letter in Word.chars().rev(){
        g.push(letter);
    } 
    g

}
fn main(){
    let word = String::from("rust");
    println!("{}",reverse(&word));
}