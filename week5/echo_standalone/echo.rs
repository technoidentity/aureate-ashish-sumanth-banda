mod echo_core;
use std::env;
fn main() {
    let words: Vec<String> = env::args()
        .skip(1)
        .collect();
    let output = echo_core::echo_text(&words);
    print!("{}", output);
}