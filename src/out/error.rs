use colored::Colorize;

// config
pub fn load_config() {
  let text = format!("Unable to load the configuration file").red();
  println!("{}", text);
}

// repository
pub fn init_repository() {
  let text = format!("Error initializing git").red();
  println!("{}", text);
}

pub fn no_repositories() {
  let text = format!("{}\nUse {} to add an repository.", "No repositories are maintained!".red(), "tmpo repository add".green());
  println!("{}", text);
}

pub fn repository_exists(repository: &str) {
  let text = format!("Repository: {} already exists", repository).red();
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
