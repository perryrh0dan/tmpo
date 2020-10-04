use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

#[test]
fn success() -> Result<(), Box<dyn std::error::Error>> {
  // Create temp directory for test
  let tmp_dir = tempfile::Builder::new()
    .tempdir_in(::std::env::current_dir().unwrap())
    .unwrap();

  let tmp_dir_path = tmp_dir.path().to_str().unwrap();

  let mut cmd = Command::cargo_bin("tmpo")?;

  cmd.arg("config");
  cmd.assert().success().stdout(predicate::str::contains("Config loaded from: /home"));

  Ok(())
}
