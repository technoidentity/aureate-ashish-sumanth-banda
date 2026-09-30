fn sum_of_multiples(limit: u32) -> u32 {
    let mut total = 0;

    for number in 1..limit {
        if number % 3 == 0 || number % 5 == 0 {
            total = total + number;
        }
    }

    total
}

fn main() {
    println!("{}", sum_of_multiples(20)); // 78
}