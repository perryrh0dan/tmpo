use colored::Colorize;

use crate::template;
use crate::utils;

pub mod errors;
pub mod warnings;

pub fn list_templates(templates: &Vec<String>) {
  for template in templates {
    println!("{}", &utils::capitalize(template));
  }
}

pub fn no_app_update() {
  let text = format!("tmpo is already up to date").green();
  println!("{}", text);
}

pub fn success_update_app() {
  let text = format!("Successful updated app").green();
  println!("{}", text);
}

pub fn success_create(name: &str) {
  let text = format!("Created workspace: {}", name).green();
  println!("{}", text);
}

pub fn display_template(template: &template::Template) {
  println!("name: {}", template.name);
  println!("path: {}", template.path);

  if !template.meta.description.is_none() {
    println!("description: {}", template.meta.description.as_ref().unwrap());
  }

  if !template.meta.extend.is_none() {
    let text = utils::vec_to_string(template.meta.extend.as_ref().unwrap());
    println!("extends: {}", text);
  }
}

pub fn initiate_workspace() {
  let text = format!("Initiate workspace").green();
  println!("{}", text);
}
