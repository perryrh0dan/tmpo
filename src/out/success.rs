use colored::Colorize;

pub fn app_updated() {
  let text = format!("Successful updated app").green();
  println!("{}", text);
}

pub fn workspace_created(name: &str) {
  let text = format!("Created workspace: {}", name).green();
  println!("{}", text);
}

pub fn workspace_info(info: &str) {
  println!("");
  println!("{}", info)
}

pub fn template_created(path: &str) {
  let text = format!("Created template: {}", path).green();
  println!("{}", text);
}

pub fn repository_added(name: &str) {
  let text = format!("Added repository: {}", name).green();
  println!("{}", text);
}

pub fn repository_removed(name: &str) {
  let text = format!("Removed repository: {}", name).green();
  println!("{}", text);
}

pub fn remote_repository_created(name: &str, target: &str) {
  let title = format!("Created remote repository: {} under: {}", name, target).green();
  let text = "Navigate into the repository and start adding new templates.\nTo use the repository push it to gitlab or github and add it via tmpo repository add.";
  println!("{}", title);
  println!("{}", text);
}

pub fn template_tested() {
  let text = format!("Test successful").green();
  println!("{}", text);
}
