use std::fmt::Debug;

#[derive(Debug)]

struct Student{
    name: String,
}
trait Summary{
    fn summarize(&self)->String;
    fn announce(&self){
        println!("Summary:{}",self.summarize());
    }
}
impl Summary for Student{
    fn summarize(&self)-> String{
        format!("Student:{}",self.name)
    }
}
fn print_summary<T:Summary + Debug>(item:&T){
    item.announce();
    println!("Debug: {:?}",item);
}
fn main(){
    let student = Student{
        name: String::from("Ashish")
    };
    print_summary(&student);
}