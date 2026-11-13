use std::env;
use std::process::Command;

fn executable() -> Command {
    let path = env::var_os("ECHO_BIN")
        .expect("set ECHO_BIN to the executable path");

    Command::new(path)
}

#[test]
fn prints_supplied_words() {
    let output = executable()
        .args(["hello", "Rust"])
        .output()
        .expect("could not run echo");

    assert!(output.status.success());
    assert_eq!(output.stdout, b"hello Rust\n");
    assert!(output.stderr.is_empty());
}

#[test]
fn prints_newline_without_arguments() {
    let output = executable()
        .output()
        .expect("could not run echo");

    assert!(output.status.success());
    assert_eq!(output.stdout, b"\n");
    assert!(output.stderr.is_empty());
}

#[test]
fn preserves_spaces_inside_one_argument() {
    let output = executable()
        .arg("hello  Rust")
        .output()
        .expect("could not run echo");

    assert!(output.status.success());
    assert_eq!(output.stdout, b"hello  Rust\n");
    assert!(output.stderr.is_empty());
}