use std::path::PathBuf;
use std::process::Command;
fn executable() -> Command {
    Command::new(env!("CARGO_BIN_EXE_word_count"))
}
fn sample_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/sample.txt")
}
#[test]
fn prints_counts_for_a_file() {
    let output = executable()
        .arg(sample_path())
        .output()
        .expect("failed to run word_count");

    assert!(output.status.success());

    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        "lines: 2\nwords: 3\nbytes: 17\n"
    );
    assert!(output.stderr.is_empty());
}
#[test]
fn reports_missing_argument() {
    let output = executable()
        .output()
        .expect("failed to run word_count");

    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());

    let error = String::from_utf8(output.stderr).unwrap();
    assert!(error.contains("usage: word_count <file>"));
}

#[test]
fn reports_unreadable_path() {
    // A regular file cannot contain another file.
    let invalid_path = sample_path().join("missing.txt");

    let output = executable()
        .arg(invalid_path)
        .output()
        .expect("failed to run word_count");

    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());

    let error = String::from_utf8(output.stderr).unwrap();
    assert!(error.contains("cannot read file:"));
}

#[test]
fn rejects_extra_arguments() {
    let output = executable()
        .arg(sample_path())
        .arg("extra")
        .output()
        .expect("failed to run word_count");

    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());

    let error = String::from_utf8(output.stderr).unwrap();
    assert!(error.contains("usage: word_count <file>"));
}