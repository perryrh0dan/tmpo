use colored::Colorize;

pub fn script_execution_failed(logs: &str) {
  let text = format!("Script execution failed: {}", logs).yellow();
  println!("{}", text);
}
