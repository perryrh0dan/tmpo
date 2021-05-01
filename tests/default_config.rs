use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

#[test]
fn success() -> Result<(), Box<dyn std::error::Error>> {
  let mut cmd = Command::cargo_bin("tmpo")?;

  cmd.arg("config");
  cmd.assert().success().stdout(predicate::str::contains("Config loaded from: /home"));

  Ok(())
}
