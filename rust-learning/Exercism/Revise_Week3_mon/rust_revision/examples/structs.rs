struct Student{
    name: String,
    score: u32 ,
}
fn main(){
    let student1 = Student{
        name: String::from("Ashish"),
        score: 80,
    };
    let student2 = Student{
        name: String::from("Sumanth"),
        score: 90,
    };
    let student3 = Student{
        name: String::from("Nawab"),
        score: 100,
    };
    println!("Name:{}", student1.name);
}