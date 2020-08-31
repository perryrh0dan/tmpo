use colored::Colorize;

// config
pub fn load_config() {
  let text = format!("Unable to load the configuration file").red();
  println!("{}", text);
}

// repository
pub fn repository_not_found(repository: &str) {
  let text = format!("Repository: {} not found", repository).red();
  println!("{}", text);
}

pub fn init_repository() {
  let text = format!("Error initializing git").red();
  println!("{}", text);
}

pub fn no_repositories() {
  let text = format!("No repositories are maintained").red();
  println!("{}", text);
}


// template
pub fn update_templates() {
  let text = format!("{}", "Error updating templates".red());
  println!("{}", text);
}

pub fn template_not_found(template: &str) {
  let text = format!("Template: {} not found", template).red();
  println!("{}", text);
}

pub fn no_templates(repository: &str) {
  let text = format!("No templates exist in repository: {}", repository).red();
  println!("{}", text);
}

// self update
pub fn selfupdate_no_permission() {
  let text = format!("Permission denied, try as root user").red();
  println!("{}", text);
}

// more
pub fn create_directory(dir: &str) {
  let text = format!("{} {}", "Cant create directory:", dir).red();
  println!("{}", text);
}

pub fn copy_template() {
  let text = format!("{}", "Cant copy template".red());
  println!("{}", text);
}

pub fn unknown() {
  let text = format!("Unknown error. See the logs for more details").red();
  println!("{}", text);
}