fn collatz(mut number: u64) -> u64 {
    assert!(number > 0, "Use a positive number");

    let mut steps = 0;

    while number != 1 {
        if number % 2 == 0 {
            number = number / 2;
        } else {
            number = number * 3 + 1;
        }

        steps += 1;
    }

    steps
}

fn main() {
    println!("{}", collatz(12)); // 9
}