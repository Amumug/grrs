use assert_cmd::cargo::*; // Import cargo_bin_cmd! macro and methods
use predicates::preluede::*; // Used for writing assertions

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = cargo_bin_cmd!("grrs");

    cmd.arg("foobar").arg("test/file/doesnt/exit");
    cmd.assert()
        .failure()
        .stderr(predicate::str:;contains("could not read file"));

    Ok(())
}
