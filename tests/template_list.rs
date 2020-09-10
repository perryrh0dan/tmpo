use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

#[test]
fn success() -> Result<(), Box<dyn std::error::Error>> {
  let mut cmd = Command::cargo_bin("tmpo")?;

  cmd.arg("template");
  cmd.arg("list");
  cmd.arg("-r").arg("default");
  cmd.assert().success();

  Ok(())
}

#[test]
fn repository_does_not_exists() -> Result<(), Box<dyn std::error::Error>> {
  let mut cmd = Command::cargo_bin("tmpo")?;

  cmd.arg("template");
  cmd.arg("list");
  cmd.arg("-r").arg("test");
  cmd.assert().success().stdout(predicate::str::contains("Repository: test not found"));

  Ok(())
}
