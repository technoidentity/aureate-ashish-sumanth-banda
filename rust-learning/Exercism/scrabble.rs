fn score(word: &str) -> u32 {
    let mut total = 0;

    for letter in word.to_ascii_lowercase().chars() {
        if "aeioulnrst".contains(letter) {
            total = total + 1;
        } else if "dg".contains(letter) {
            total = total + 2;
        } else if "bcmp".contains(letter) {
            total = total + 3;
        } else if "fhvwy".contains(letter) {
            total = total + 4;
        } else if letter == 'k' {
            total = total + 5;
        } else if "jx".contains(letter) {
            total = total + 8;
        } else if "qz".contains(letter) {
            total = total + 10;
        }
    }

    total
}

fn main() {
    println!("{}", score("rust")); // 4
}