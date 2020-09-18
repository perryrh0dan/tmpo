use colored::Colorize;

pub fn no_repositories() {
  let text = format!(
    "{}\nUse {} to add an repository.",
    "No repositories are maintained!".red(),
    "tmpo repository add".green()
  );
  eprintln!("{}", text);
}

pub fn no_templates(repository_name: &str) {
  let text = format!("No templates found in repository: {}", repository_name);
  eprintln!("{}", text);
}

pub fn template_not_found(template: &str) {
  let text = format!("Template: {} not found", template);
  eprintln!("{}", text);
}

pub fn repository_exists(repository: &str) {
  let text = format!("Repository: {} already exists", repository);
  eprintln!("{}", text);
}

pub fn template_exists(template: &str) {
  let text = format!("Template: {} already exists", template);
  eprintln!("{}", text);
}
