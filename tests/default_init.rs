use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

#[test]
fn repository_does_not_exists() -> Result<(), Box<dyn std::error::Error>> {
  let mut cmd = Command::cargo_bin("tmpo")?;

  cmd.arg("init").arg("name");
  cmd.arg("-r").arg("tttt");
  cmd.assert().failure().stderr(predicate::str::contains("Unable to load repository! Error: Not found"));

  Ok(())
}

#[test]
fn template_does_not_exists() -> Result<(), Box<dyn std::error::Error>> {
  let mut cmd = Command::cargo_bin("tmpo")?;

  cmd.arg("init").arg("name");
  cmd.arg("-r").arg("default");
  cmd.arg("-t").arg("test");
  cmd.assert().failure().stderr(predicate::str::contains("Unable to load template! Error: Not found"));

  Ok(())
}

#[test]
fn success() -> Result<(), Box<dyn std::error::Error>> {
  // Create temp directory for test
  let tmp_dir = tempfile::Builder::new()
    .tempdir_in(::std::env::current_dir().unwrap())
    .unwrap();

  let tmp_dir_path = tmp_dir.path().to_str().unwrap();

  let mut cmd = Command::cargo_bin("tmpo")?;

  cmd.arg("init").arg("name");
  cmd.arg("-r").arg("default");
  cmd.arg("-t").arg("golang");
  cmd.arg("-d").arg(tmp_dir_path);
  cmd.arg("--remote").arg("github.com");
  cmd.assert().success().stdout(predicate::str::contains("Created workspace: name"));

  Ok(())
}
