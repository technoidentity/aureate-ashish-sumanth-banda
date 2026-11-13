struct Student<'a>{
    name: &'a str,
}
impl<'a> Student<'a>{
    fn get_name(&self)->&str{
        self.name
    }
}
fn main(){
    let original_name = String::from("Ashish");
    let student = Student{
        name: &original_name,
    };
    println!("Student:{}",student.get_name());
    println!("Original: {}",original_name);
}