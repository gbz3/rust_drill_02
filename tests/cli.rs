use assert_cmd::Command;
// use predicates::prelude::*;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn input0() -> TestResult {
    let mut cmd = Command::cargo_bin("rust_drill_02")?;
    cmd.assert().success().stdout("\n");
    Ok(())
}

fn run(args: &[&str], expected: &'static str) -> TestResult {
    let mut cmd = Command::cargo_bin("rust_drill_02")?;
    cmd.args(args).assert().success().stdout(expected);
    Ok(())
}

#[test]
fn input0_n() -> TestResult {
    run(&["-n"], "")
}

#[test]
fn input1() -> TestResult {
    run(&["Hello"], "Hello\n")
}

#[test]
fn input1_n_lead() -> TestResult {
    run(&["-n", "Hello"], "Hello")
}

#[test]
fn input1_n_last() -> TestResult {
    run(&["Hello", "-n"], "Hello")
}

#[test]
fn input2() -> TestResult {
    run(&["Hello", "world"], "Hello world\n")
}

#[test]
fn input2_n_lead() -> TestResult {
    run(&["-n", "Hello", "world"], "Hello world")
}

#[test]
fn input2_n_middle() -> TestResult {
    run(&["Hello", "-n", "world"], "Hello world")
}

#[test]
fn input2_n_last() -> TestResult {
    run(&["Hello", "world", "-n"], "Hello world")
}

