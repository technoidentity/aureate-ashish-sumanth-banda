fn show(text: &String ){
    println! ("{text}");
}

fn add_words(text: &mut String){
    text.push_str("is learning rust");
}

fn main(){
    let mut name = String :: from("Ashish");
    add_words(&mut name);
    let reader = &name;
    println!("{name}");
    println!("{reader}");
}