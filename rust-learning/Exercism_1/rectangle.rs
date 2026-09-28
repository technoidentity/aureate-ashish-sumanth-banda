struct Rectangle{
    width: u32,
    height : u32
}
impl Rectangle{
    fn area(&self)->u32{
        let a = self.width * self.height;
        a
    }
}
fn main(){
    let rectangle = Rectangle{
        width: 3,
        height: 4,
    };
    let result = rectangle.area();
    println!("{result}");
}