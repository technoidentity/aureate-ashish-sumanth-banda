mod core;

use std::{env, fs, process};

fn run() -> Result<(), String> {
    let mut args = env::args_os().skip(1);

    let path = args
        .next()
        .ok_or_else(|| String::from("usage: word_count <file>"))?;

    if args.next().is_some() {
        return Err(String::from("usage: word_count <file>"));
    }

    let text = fs::read_to_string(path)
        .map_err(|error| format!("cannot read file: {error}"))?;

    let counts = core::count_text(&text);

    println!("lines: {}", counts.lines);
    println!("words: {}", counts.words);
    println!("bytes: {}", counts.bytes);

    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("error: {error}");
        process::exit(1);
    }
}