use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

#[test]
fn success() -> Result<(), Box<dyn std::error::Error>> {
  let mut cmd = Command::cargo_bin("tmpo")?;

  cmd.arg("repository");
  cmd.arg("view");
  cmd.arg("-r").arg("default");
  cmd.assert().success();

  Ok(())
}

#[test]
fn repository_does_not_exists() -> Result<(), Box<dyn std::error::Error>> {
  let mut cmd = Command::cargo_bin("tmpo")?;

  cmd.arg("repository");
  cmd.arg("view");
  cmd.arg("-r").arg("tttt");
  cmd.assert().failure().stderr(predicate::str::contains("Unable to load repository! Error: Not found"));

  Ok(())
}
