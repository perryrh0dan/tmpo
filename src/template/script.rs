use std::path::Path;
use std::process::{Command, Stdio};

use crate::logger;
use crate::out;

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
      let logfile_path = logger::get_log_file_path().into_os_string().into_string().unwrap();
      out::warn::script_execution_failed(&logfile_path);
      log::error!("Script exited with error: {}", error);
      return;
    }
  };

  log::info!("Script exit status: {}", status);
}
