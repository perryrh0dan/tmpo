use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

// #[test]
// fn success() -> Result<(), Box<dyn std::error::Error>> {
//   let mut cmd = Command::cargo_bin("tmpo")?;

//   cmd.arg("repository");
//   cmd.arg("add");
//   cmd.arg("-n").arg("templates");
//   cmd.arg("-d").arg("Just a repository");
//   cmd.assert().success();

//   Ok(())
// }

#[test]
fn remote_repository_already_exists() -> Result<(), Box<dyn std::error::Error>> {
  let mut cmd = Command::cargo_bin("tmpo")?;

  cmd.arg("repository");
  cmd.arg("add");
  cmd.arg("-t").arg("remote");
  cmd.arg("-n").arg("templates");
  cmd.arg("-d").arg("just a repository");
  cmd.arg("--provider").arg("github");
  cmd.arg("--authentication").arg("none");
  cmd.arg("--url").arg("https://github.com/perryrh0dan/templates");
  cmd.arg("--branch").arg("master");
  cmd.assert().failure().stderr(predicate::str::contains("Repository: templates already exists"));

  Ok(())
}
