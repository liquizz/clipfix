use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn em_dash_hard_mode() {
    let mut cmd = Command::cargo_bin("clipfix").unwrap();
    cmd.arg("--hard")
        .write_stdin("Hello \u{2014} world\n")
        .assert()
        .success()
        .stdout("Hello -- world\n");
}

#[test]
fn em_dash_soft_mode_preserved() {
    let mut cmd = Command::cargo_bin("clipfix").unwrap();
    cmd.write_stdin("Hello \u{2014} world\n")
        .assert()
        .success()
        .stdout("Hello \u{2014} world\n");
}

#[test]
fn curly_quotes_hard_mode() {
    let mut cmd = Command::cargo_bin("clipfix").unwrap();
    cmd.arg("--hard")
        .write_stdin("He said \u{201C}hello\u{201D}\n")
        .assert()
        .success()
        .stdout(predicate::str::contains("He said \"hello\"\n"));
}

#[test]
fn curly_quotes_soft_mode_preserved() {
    let mut cmd = Command::cargo_bin("clipfix").unwrap();
    cmd.write_stdin("He said \u{201C}hello\u{201D}\n")
        .assert()
        .success()
        .stdout("He said \u{201C}hello\u{201D}\n");
}

#[test]
fn soft_mode_removes_zero_width_space() {
    let mut cmd = Command::cargo_bin("clipfix").unwrap();
    cmd.write_stdin("foo\u{200B}bar\n")
        .assert()
        .success()
        .stdout("foobar\n");
}

#[test]
fn soft_mode_removes_bom() {
    let mut cmd = Command::cargo_bin("clipfix").unwrap();
    cmd.write_stdin("\u{FEFF}hello\n")
        .assert()
        .success()
        .stdout("hello\n");
}

#[test]
fn soft_flag_explicit_preserves_em_dash() {
    let mut cmd = Command::cargo_bin("clipfix").unwrap();
    cmd.arg("-S")
        .write_stdin("dash \u{2014} here\n")
        .assert()
        .success()
        .stdout("dash \u{2014} here\n");
}

#[test]
fn hard_flag_short_replaces_em_dash() {
    let mut cmd = Command::cargo_bin("clipfix").unwrap();
    cmd.arg("-H")
        .write_stdin("dash \u{2014} here\n")
        .assert()
        .success()
        .stdout("dash -- here\n");
}

#[test]
fn soft_and_hard_flags_conflict() {
    let mut cmd = Command::cargo_bin("clipfix").unwrap();
    cmd.args(["--soft", "--hard"])
        .write_stdin("text\n")
        .assert()
        .failure();
}

#[test]
fn list_replacements_flag() {
    let mut cmd = Command::cargo_bin("clipfix").unwrap();
    cmd.arg("--list-replacements")
        .assert()
        .success()
        .stdout(predicate::str::contains("Soft mode replacements"))
        .stdout(predicate::str::contains("Hard mode replacements"))
        .stdout(predicate::str::contains("U+2014"));
}

#[test]
fn list_replacements_short_flag() {
    let mut cmd = Command::cargo_bin("clipfix").unwrap();
    cmd.arg("-l")
        .assert()
        .success()
        .stdout(predicate::str::contains("U+00A0"));
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
fn help_mentions_soft_and_hard() {
    let mut cmd = Command::cargo_bin("clipfix").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("--soft"))
        .stdout(predicate::str::contains("--hard"))
        .stdout(predicate::str::contains("--list-replacements"));
}

#[test]
fn version_flag() {
    let mut cmd = Command::cargo_bin("clipfix").unwrap();
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("0.1.2"));
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
fn ellipsis_and_em_dash_hard_mode() {
    let mut cmd = Command::cargo_bin("clipfix").unwrap();
    cmd.arg("--hard")
        .write_stdin("Wait\u{2026} \u{2014} now\n")
        .assert()
        .success()
        .stdout("Wait... -- now\n");
}

#[test]
fn ellipsis_and_em_dash_soft_mode_preserved() {
    let mut cmd = Command::cargo_bin("clipfix").unwrap();
    cmd.write_stdin("Wait\u{2026} \u{2014} now\n")
        .assert()
        .success()
        .stdout("Wait\u{2026} \u{2014} now\n");
}
