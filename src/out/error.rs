use colored::Colorize;

pub fn no_repositories() {
  let text = format!("{}\nUse {} to add an repository.", "No repositories are maintained!".red(), "tmpo repository add".green());
  eprintln!("{}", text);
}

pub fn repository_exists(repository: &str) {
  let text = format!("Repository: {} already exists", repository);
  eprintln!("{}", text);
}
