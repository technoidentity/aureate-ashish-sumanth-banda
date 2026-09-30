fn first_long(names: &[String])-> Option<&String>{
    for name in names{
        if name.chars().count()>5{
            return Some(name);
        }
    }
    None
}
fn main(){
    let names = [
        String::from("pen"),
        String::from("apple"),
        String::from("bag")];
    let answer = first_long(&names);
    match answer{
        Some(name)=>println!("{}",name),
        None => println!("No long name found"),
    }
}