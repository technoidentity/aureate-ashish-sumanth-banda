fn largest_in_array(numbers: [i32;5])->i32{
    let mut current = numbers[0];
    for number in numbers{
        
        if number > current{
            current = number;
        }      
    }
    return current
}

fn main(){
    let num = [4,7,2,9,6];
    println!("{}",largest_in_array(num));
}
