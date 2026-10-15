enum Command {
    Add(String),
    Remove(usize),
    List,
    Clear,
}

// TODO 1 — return a description for every variant.
//   Add(milk) -> "add milk"      Remove(2) -> "remove item 2"
//   List      -> "list items"    Clear     -> "clear the list"
fn describe(c: &Command) -> String {
    match c{
        Command::Add(item)=>{
            let mut description = String::from("add");
            description.push_str(item);
            description
    }
        Command::Remove(index)=>{
            let mut description = String::from("remove item");
            let number_text = index.to_string( );
            description.push_str(&number_text);
            description
        }
        Command::List=> String::from("list items"),
        Command::Clear=> String::from("clear the list"),
    }
}

// TODO 2 — return the first name longer than five characters, or None.
fn first_long(names: &[String]) -> Option<&String> {
    for name in names{
        if name.chars().count()>5{
            return Some(name);
        }
    }
    None
}

fn main() {
    let cmds = [
        Command::Add(String::from("milk")),
        Command::Remove(2),
        Command::List,
        Command::Clear,
    ];
    for c in &cmds {
        println!("{}", describe(c));
    }

    let names = [String::from("pen"), String::from("notebook"), String::from("bag")];
    match first_long(&names) {
        Some(s) => println!("first long name: {}", s),
        None => println!("nothing longer than five"),
    }
}