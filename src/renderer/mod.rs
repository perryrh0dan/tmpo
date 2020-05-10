use colored::Colorize;

use crate::utils;

pub mod errors;
pub mod warnings;

pub fn list_templates(templates: &Vec<String>) {
  for template in templates {
    println!("{}", &utils::capitalize(template));
  }
}

pub fn success_create(name: &str) {
  let text = format!("Created workspace: {}", name).green();
  println!("{}", text);
}

pub fn check_template_updates() {
  let text = format!("Check for template updates");
  println!("{}", text);
}

pub fn no_template_updates() {
  let text = format!("No updates found").green();
  println!("{}", text);
}

pub fn success_update_templates() {
  let text = format!("Successful updated templates").green();
  println!("{}", text);
}
