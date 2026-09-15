
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop{
   	 println!("Enter your guess:");

   	 let mut guess = String::new();

   	 io::stdin()
		.read_line(&mut guess)
		.expect("Failed to read input");

    	 let guess: u32 = match guess.trim().parse(){
                Ok(number) => number,
		Err(_) => {
			println!("please enter a valid whole number.");
			continue;
		}
	};


	println!("your guess : {guess}");
	match guess.cmp(&secret_number){
		Ordering::Less => println!("Too small!"),
		Ordering::Greater => println!("Too big!"),
		Ordering::Equal => {
			println!("you win!");
			break;
		}
	   }

      }

}
