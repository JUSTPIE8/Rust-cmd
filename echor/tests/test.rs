use assert_cmd::Command;
type TestResult = Result<(), Box<dyn std::error::Error>>;
fn run(args: &[&str], expected: &'static str) -> TestResult {
    Command::cargo_bin("echor")?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}
#[test]
fn hello1() -> TestResult {
    run(&["hello there"], "hello there\n")
}
#[test]
fn hello2() -> TestResult {
    run(&["hello", "there"], "hello there\n")
}

#[test]
fn no_newline() -> TestResult {
    run(&["hello there", "-n"], "hello there")
}
