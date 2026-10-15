struct Book{
    title:String,
    pages:u32,
}
fn main(){
    let book1= Book{
        title: String::from ("Rust Basic"),
        pages:  120,
    };
    let book2= Book{
        title: String::from("Rust Practice"),
        pages: 200,
    };
    if book1.pages > book2.pages{
        println!("{}",book1.title);
    }
    else if book2.pages > book1.pages {
        println!("{}",book2.title);
    }
    else {
        println!("Both books have the same number of pages");
    }
    
}