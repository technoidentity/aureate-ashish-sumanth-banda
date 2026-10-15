fn pass_on(mut text: String) -> String {
    text.push_str("is learning rust");
    text
}

fn main() {
    let name = String::from("Ashish ");
    let returned = pass_on(name);
    println!("{returned}");
}