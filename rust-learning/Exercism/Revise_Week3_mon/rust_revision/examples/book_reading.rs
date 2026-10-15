struct Book{
    title : String,
    pages : u32,
}

impl Book{
    fn reading_time(&self)->u32{
        let reading_time = self.pages * 2;
        reading_time
    }
}
fn main(){
    let book = Book{
        title : String::from("its rust baby"),
        pages : 35,
    };
    let time = book.reading_time();
    println!("{}",time);
}