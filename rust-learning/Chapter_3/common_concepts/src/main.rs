const PASS_MARK: i32 =50;
fn main(){

	let score =40;
	let score = score + 15;

	{
		let score = score*2;
		println!("Inner score: {score}");
	}
	println!("outer score: {score}");
	
	let mut attempts = 1;
	attempts += 1;
	println!("attempts: {attempts}");

	let spaces = "    ";
	let spaces = spaces.len();
	println!("spaces:{spaces}");
	
	//data types
	
	let temperature: f64 = 36.5;
	let active: bool = true;
	let badge: char = 'R';

	let learner: (char,i32, bool) = (badge,score,active);
	let (letter,points, ready) = learner;
	println!("Tuple: {letter}, {points}, {ready}");
	println!("Tuple first: {}", learner.0);
	
	let marks: [i32;3] = [40,55,80];
	let repeated = [7; 3];
	println!("first mark:{}; repeated: {repeated:?}", marks[0]);
	println!("Temperature: {temperature}");
	println!("Division: {}, {}", 5/2, 5.0/2.0);
	println!("Remainder: {}", 5%2);
	
	//functions and expressions
	let boosted = add_bonus(score,5);
	let doubled = {
		let base =4;
		base*2
	};
	println!("Boosted: {boosted}; block: {doubled}");
	
	//conditions and loops
	
	let label = if boosted >= 80{
		"excellent"
	} else if boosted >= PASS_MARK{
		"pass"
	} else {
	        "retry"
	};
	println!("Result: {label}");

	for mark in marks{
		if mark < PASS_MARK{
			continue;
		}
		println!("Passing mark: {mark}");
	}
	let mut remaining = 2;
	while remaining > 0{
		println!("While:{remaining}");
		remaining -= 1;
	}
	let mut tries = 0;
	let reward = loop{
		tries +=1;
		if tries ==3{
			break tries * 10;
		}
	};
	println!("Loop reward: {reward}");
	
	for number in (1..4).rev(){
		println!("countdown:{number}");
	}
	
	'rows: for row in 1..=2{
		for column in 1..=3{
			if row == 2 && column ==2{
				break 'rows;
			}
			println!("cell:{row},{column}");
		}
	}
}

fn add_bonus(score: i32, bonus: i32) -> i32{
	score+bonus
}
