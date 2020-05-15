use colored::Colorize;

use crate::repository::template;
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

pub fn display_template(template: &template::Template) {
  println!("name: {}", template.name);
  println!("path: {}", template.path);

  if !template.description.is_none() {
    println!("description: {}", template.description.as_ref().unwrap());
  }

  if !template.extend.is_none() {
    let text = utils::vec_to_string(template.extend.as_ref().unwrap());
    println!("extends: {}", text);
  }
}

pub fn initiate_workspace(name: &String) {
  println!("Initiate workspace: {}", name);
}
