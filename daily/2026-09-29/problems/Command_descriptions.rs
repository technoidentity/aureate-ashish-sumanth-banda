enum Command{
    Add(String),
    Remove(usize),
    List,
    Clear,
}
fn describe(command:&Command)->String{
    match command{
        Command::Add(item)=>{
            let mut description = String :: from("add ");
            description.push_str(item);
            description
        },
        Command::Remove(item)=>{
            let mut description = String::from("Remove item ");
            let number_text = item.to_string();
            description.push_str(&number_text);
            description
        },
        Command::List=>String::from("list items"),
        Command::Clear=>String::from("clear the list"),
    }

}
fn main(){
    let command = Command::Add(String::from("bread"));
    let description =describe(&command);
    println!("{description}");
    let command = Command::Remove(5);
    let description = describe(&command);
    println!("{description}");
    let command = Command::List;
    let description = describe(&command);
    println!("{description}");
    let command = Command::Clear;
    let description = describe(&command);
    println!("{description}");
}
