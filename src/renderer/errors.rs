use colored::Colorize;

pub fn update_templates() {
  let text = format!("{}", "Error updating templates".red());
  println!("{}", text);
}

pub fn repository_not_found(repository: &str) {
  let text = format!("Repository: {} not found", repository).red();
  println!("{}", text);
}

pub fn template_not_found(template: &str) {
  let text = format!("Template: {} not found", template).red();
  println!("{}", text);
}

pub fn create_directory(dir: &str) {
  let text = format!("{} {}", "Cant create directory:", dir).red();
  println!("{}", text);
}

pub fn copy_template() {
  let text = format!("{}", "Cant copy template".red());
  println!("{}", text);
}

pub fn template_dir_not_found(dir: &str) {
  let text = format!("{} {}", "Cant find template dir:", dir).red();
  println!("{}", text);
}

pub fn template_dir_permission_denied(dir: &str) {
  let text = format!("{} {}", "Cant read template dir:", dir).red();
  println!("{}", text);
}

pub fn init_repository() {
  let text = format!("{}", "Error initializing git".red());
  println!("{}", text);
}

pub fn missing_token() {
  let text = format!("Error fetching templates: authentication token is missing").red();
  println!("{}", text);
}

pub fn unknown() {
  let text = format!("Unknown error occured").red();
  println!("{}", text);
}
