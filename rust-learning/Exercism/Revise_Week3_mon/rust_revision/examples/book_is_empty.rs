struct Book{
    title : String,
    pages : u32,
}
impl Book{
    fn is_empty(&self) -> bool{
        if self.pages == 0{
            true
        }
        else{
            false
        }
    }
}
fn main(){
    let book = Book{
        title : String::from("Bible"),
        pages : 0, 
    };
    let value = book.is_empty();
    println!("{}",value);
}