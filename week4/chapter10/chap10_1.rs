struct Pair<T>{
    first: T,
    second: T,
}
impl<T> Pair<T>{
    fn get_first(&self)-> &T{
        &self.first
    }
}
impl<T: PartialOrd> Pair<T>{
    fn larger(&self)-> &T{
        if self.first > self.second{
            &self.first
        }else{
            &self.second
        }
    }
}

fn main() {
    let numbers = Pair {
        first: 10,
        second: 25,
    };

    let letters = Pair {
        first: 'a',
        second: 'z',
    };

    println!("Larger number: {}", numbers.larger());
    println!("Larger letter: {}", letters.larger());
    println!("First number: {}", numbers.get_first());
}