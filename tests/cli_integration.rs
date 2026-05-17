use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn em_dash_piped_input() {
    let mut cmd = Command::cargo_bin("clipfix").unwrap();
    cmd.write_stdin("Hello \u{2014} world\n")
        .assert()
        .success()
        .stdout("Hello -- world\n");
}

#[test]
fn curly_quotes_input() {
    let mut cmd = Command::cargo_bin("clipfix").unwrap();
    cmd.write_stdin("He said \u{201C}hello\u{201D}\n")
        .assert()
        .success()
        .stdout(predicate::str::contains("He said \"hello\"\n"));
}

#[test]
fn ascii_passthrough() {
    let mut cmd = Command::cargo_bin("clipfix").unwrap();
    cmd.write_stdin("ASCII only\n")
        .assert()
        .success()
        .stdout("ASCII only\n");
}

#[test]
fn empty_input() {
    let mut cmd = Command::cargo_bin("clipfix").unwrap();
    cmd.write_stdin("\n").assert().success().stdout("\n");
}

#[test]
fn help_flag() {
    let mut cmd = Command::cargo_bin("clipfix").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("clipfix"));
}

#[test]
fn version_flag() {
    let mut cmd = Command::cargo_bin("clipfix").unwrap();
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("0.1.0"));
}

#[test]
fn unknown_flag() {
    let mut cmd = Command::cargo_bin("clipfix").unwrap();
    cmd.arg("--unknown-flag").assert().failure();
}

#[test]
fn clipboard_flag_recognized() {
    let mut cmd = Command::cargo_bin("clipfix").unwrap();
    cmd.arg("--clipboard")
        .arg("--help")
        .assert()
        .stdout(predicate::str::contains("clipboard"));
}

#[test]
fn ellipsis_and_em_dash_combination() {
    let mut cmd = Command::cargo_bin("clipfix").unwrap();
    cmd.write_stdin("Wait\u{2026} \u{2014} now\n")
        .assert()
        .success()
        .stdout("Wait... -- now\n");
}
