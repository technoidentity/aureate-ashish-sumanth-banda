fn reverse(text: &str)->String{
    let mut reversed = String::new();
    for letter in text.chars().rev(){
        reversed.push(letter);
    }
    reversed
}
fn main(){
    println!("{}", reverse("cat"))
}