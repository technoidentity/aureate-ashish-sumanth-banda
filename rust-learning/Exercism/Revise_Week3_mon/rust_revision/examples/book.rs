struct Book{
    title : String,
    pages : u32,

}
fn main(){
    let book = Book {
        title : String::from("Rust Basics"),
        pages : 120,
    };
    println!("{}",book.title);
    println!("{}",book.pages);
}