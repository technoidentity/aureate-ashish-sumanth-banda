struct Student{
    name:String,
    score:u32,
}
enum Outcome{
    Pass,
    Fail,
}
impl Student{
    fn build(name:String,score:u32)->Option<Student>{
        if score <= 100{
            Some(Student{name,score})
        }
        else{
            None
        }
    }
    fn outcome(&self)->Outcome{
        if self.score >= 40{
            Outcome::Pass
        }
        else{
            Outcome::Fail
        }
    }
}
fn main(){
    let student1 = Student::build("Ashish".to_string(),50);
    match student1{
        Some(item)=>{
            println!("{}",item.name);
            match item.outcome(){
                Outcome::Pass=>println!("Pass"),
                Outcome::Fail=>println!("Fail")
            }
        }
        None=>{
            println!("Invalid Score");
        }
    }
}