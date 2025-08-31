use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

#[test]
fn fails_with_no_args() {
    let mut cmd = Command::cargo_bin("minigrep").unwrap();
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Not Enough Arguments"));
}

#[test]
fn finds_case_sensitive_match() {
    let mut cmd = Command::cargo_bin("minigrep").unwrap();

    let filename = "tests/input.txt";
    fs::write(
        filename,
        "Rust:\nsafe, fast, productive.\nPick three.\nDuct tape.",
    )
    .unwrap();

    cmd.args(["duct", filename])
        .assert()
        .success()
        .stdout(predicate::str::contains("safe, fast, productive."));
}

#[test]
fn finds_case_insensitive_match() {
    let mut cmd = Command::cargo_bin("minigrep").unwrap();

    let filename = "tests/input_insensitive.txt";
    fs::write(
        filename,
        "Rust:\nsafe, fast, productive.\nPick three.\nTrust me.",
    )
    .unwrap();

    cmd.args(["rUsT", filename, "-i"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Rust:"))
        .stdout(predicate::str::contains("Trust me."));
}

#[test]
fn invert_match_works() {
    let mut cmd = Command::cargo_bin("minigrep").unwrap();

    let filename = "tests/input_invert.txt";
    fs::write(
        filename,
        "Rust:\nsafe, fast, productive.\nPick three.\nTrust me.",
    )
    .unwrap();

    cmd.args(["Rust", filename, "-v"])
        .assert()
        .success()
        .stdout(predicate::str::contains("safe, fast, productive."))
        .stdout(predicate::str::contains("Pick three."));
}

#[test]
fn count_only_flag() {
    let mut cmd = Command::cargo_bin("minigrep").unwrap();

    let filename = "tests/input_count.txt";
    fs::write(filename, "alpha\nbeta\ngamma\nalpha\n").unwrap();

    cmd.args(["alpha", filename, "-c"])
        .assert()
        .success()
        .stdout(predicate::str::contains("2")); // alpha occurs twice
}

#[test]
fn line_number_flag() {
    let mut cmd = Command::cargo_bin("minigrep").unwrap();

    let filename = "tests/input_line_number.txt";
    fs::write(filename, "line one\nsecond line\nanother one\nlast line\n").unwrap();

    cmd.args(["line", filename, "-n"])
        .assert()
        .success()
        .stdout(predicate::str::contains("1:line one"))
        .stdout(predicate::str::contains("2:second line"))
        .stdout(predicate::str::contains("4:last line"));
}
