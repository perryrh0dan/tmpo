use std::path::Path;
use std::process::{Command, Stdio};

pub fn run(script: &String, target: &Path) {
  // Check if script is empty
  if script == "" {
    return;
  }

  log::info!("Run script: {}", script);

  let mut cmd = if cfg!(target_os = "windows") {
    Command::new("cmd")
      .current_dir(target)
      .arg("/C")
      .arg(script)
      .stdout(Stdio::inherit())
      .stderr(Stdio::inherit())
      .spawn()
      .expect("failed to execute process")
  } else {
    Command::new("sh")
      .current_dir(target)
      .arg("-c")
      .arg(script)
      .stdout(Stdio::inherit())
      .stderr(Stdio::inherit())
      .spawn()
      .expect("failed to execute process")
  };

  let status = match cmd.wait() {
    Ok(status) => status,
    Err(error) => {
      log::error!("Script exited with error: {}", error);
      return;
    }
  };

  log::info!("Script exit status: {}", status);
}
