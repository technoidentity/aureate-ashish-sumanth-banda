struct Book{
    title : String,
    pages : u32
}
fn show_book(book: &Book){
    println!("Books title and page count {},{}",book.title, book.pages);
}
fn main(){
    let book = Book{
        title : String :: from("Rust Basic"),
        pages : 150,    
    };
    show_book(&book);
    println!("{}",book.pages);
}