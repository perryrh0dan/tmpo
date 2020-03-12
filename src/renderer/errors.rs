use colored::Colorize;

pub fn update_templates() {
    let text = format!("{}", "Error updating templates".red());
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

pub fn unknown() {
    let text = format!("Unknown error occured").red();
    println!("{}", text);
}