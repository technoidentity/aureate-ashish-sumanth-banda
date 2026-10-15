struct Book{
    title : String,
    pages : u32,
}
impl Book{
    fn is_long(&self) -> bool{
        if self.pages >= 200{
            true
        }
        else{
            false
        }
    }
}
fn main(){
    let book = Book{
        title : String::from("Rust Basics"),
        pages : 150,
    };
    let b = book.is_long();
    println!("{}",b);
}