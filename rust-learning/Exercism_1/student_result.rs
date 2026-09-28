struct Student{
    name: String,
    score: u32,
}
enum Outcome{
    Pass,
    Fail,
}
impl Student{
    fn outcome(&self)-> Outcome{
        if self.score >= 40{
            Outcome::Pass
        }
        else{
            Outcome::Fail
        }
    }
}
fn main(){
    let Student1 = Student{
        name: String::from("Asha"),
        score: 75,
    };
    let Student2 = Student{
        name : String::from("Ravi"),
        score: 32,
    };
    let result1 = Student1.outcome();
    let result2 = Student2.outcome();
    match result1{
        Outcome::Pass => println!("{}:Pass",Student1.name),
        Outcome::Fail => println!("{}:Fail",Student1.name),
    }
    match result2{
        Outcome::Pass => println!("{}:Pass",Student2.name),
        Outcome::Fail => println!("{}:Fail",Student2.name),
    }
}