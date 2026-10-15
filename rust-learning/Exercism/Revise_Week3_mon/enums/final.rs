struct Student{
    name : String,
    score : u32,
}
enum Outcome{
    pass,
    fail,
}
impl Student{
    fn out_come(&self)->Outcome{
        if self.score >= 40{
            Outcome::pass
        }
        else{
            Outcome::fail
        }
    }

}
fn main(){
    let student1 = Student{
        name : String::from("Asish"),
        score : 50,
    };
    let student2 = Student{
        name : String::from("Ravi"),
        score : 30,
    };
    let one = student1.out_come();
    let two = student2.out_come();
    match one{
        Outcome :: pass => println!("{}: pass",student1.name),
        Outcome :: fail => println!("{}: fail",student1.name),
    }
    match two {
        Outcome :: pass => println!("{}: pass",student2.name),
        Outcome :: fail => println!("{}: fail",student2.name),
    }
}