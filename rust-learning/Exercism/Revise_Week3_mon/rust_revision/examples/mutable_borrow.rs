fn add_exclamation(text: &mut String){
    text.push('!');

}
fn main(){
    let mut message = String :: from("Hello");
    add_exclamation(&mut message);
    println!("{}",message);
}