use std::env;
use std::path::PathBuf;
use std::process::Command;

fn executable() -> Command {
    let path = env::var_os("WORD_COUNT_BIN")
        .expect("set WORD_COUNT_BIN to the compiled program's path");

    Command::new(path)
}

fn sample_path() -> PathBuf {
    env::current_dir()
        .expect("cannot read current directory")
        .join("sample.txt")
}

#[test]
fn prints_correct_counts() {
    let output = executable()
        .arg(sample_path())
        .output()
        .expect("could not run word_count");

    assert!(output.status.success());
    assert!(output.stderr.is_empty());

    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        "lines: 2\nwords: 3\nbytes: 17\n"
    );
}

#[test]
fn rejects_missing_argument() {
    let output = executable()
        .output()
        .expect("could not run word_count");

    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());

    let error = String::from_utf8(output.stderr).unwrap();
    assert!(error.contains("usage: word_count <file>"));
}

#[test]
fn rejects_extra_arguments() {
    let output = executable()
        .arg(sample_path())
        .arg("extra")
        .output()
        .expect("could not run word_count");

    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());

    let error = String::from_utf8(output.stderr).unwrap();
    assert!(error.contains("usage: word_count <file>"));
}

#[test]
fn reports_file_read_failure() {
    let invalid_path = sample_path().join("missing.txt");

    let output = executable()
        .arg(invalid_path)
        .output()
        .expect("could not run word_count");

    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());

    let error = String::from_utf8(output.stderr).unwrap();
    assert!(error.contains("cannot read file:"));
}