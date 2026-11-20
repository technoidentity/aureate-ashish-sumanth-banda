trait Speak {
    fn speak(&self){
        println!("Hello!");
    }
}

struct Robot;
struct Person;

impl Speak for Robot {
    fn speak(&self) {
        println!("Beep!");
    }
}

impl Speak for Person {}

fn make_speak<T>(value: &T) 
where
    T:Speak,
{
    value.speak();
}

fn main() {
    let robot = Robot;
    let person = Person;

    make_speak(&robot);
    make_speak(&person);
}