struct Student {
    name: String,
}

trait Summary {
    fn summarize(&self) -> String;
}

impl Summary for Student {
    fn summarize(&self) -> String {
        format!("Student: {}", self.name)
    }
}

fn main() {
    let student = Student {
        name: String::from("Ashish"),
    };

    let text = student.summarize();

    println!("{}", text);
}