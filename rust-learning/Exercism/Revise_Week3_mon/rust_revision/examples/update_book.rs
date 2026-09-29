struct Book{
    title: String,
    pages: u32,
}
fn main(){
    let mut book = Book{
        title : String::from("Rust Basics"),
        pages : 120, 
    };
    println!("{}",book.pages);
    book.pages = 30 + book.pages;
    println!("{}", book.pages);
}