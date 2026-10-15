struct Student{
    name: String,
    score: u32,
}
enum Outcome{
    Pass,
    Fail,
}
impl Student{
    fn outcome(&self)->Outcome{
        if self.score >= 40{
            Outcome::Pass
        }
        else{
            Outcome::Fail
        }
    }
    fn first_passing(students: &[Student])->Option<&Student>{
        for student in students{
            match student.outcome(){
                Outcome::Pass => return Some(student), 
                Outcome::Fail => continue,
            }
            
        }
        None
    }
    
}
fn main(){
    let students=[
        Student{
            name : String::from("Asha"),
            score : 35
        },
        Student{
            name: String::from("Ravi"),
            score: 70,
        },
        Student{
            name: String::from("Lee"),
            score: 95,
        },

    ];
    let result = Student::first_passing(&students);
    match result{
        Some(student)=> println!("{}",student.name),
        None => println!("No passing student"),
    }


}