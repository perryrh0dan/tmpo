use crate::config::Config;
use crate::repository::Repository;
use crate::template::Template;
use crate::utils;

use colored::Colorize;

pub fn initiate_workspace() {
  let text = format!("Initiate workspace").green();
  println!("{}", text);
}

pub fn list_templates(templates: &Vec<String>) {
  for template in templates {
    println!("{}", &utils::capitalize(template));
  }
}

pub fn list_repositories(repositories: &Vec<String>) {
  for repository in repositories {
    println!("{}", &utils::capitalize(repository));
  }
}

pub fn display_template(template: &Template) {
  println!("name: {}", template.name);
  println!("path: {}", template.path.to_str().unwrap());

  if !template.meta.description.is_none() {
    println!(
      "description: {}",
      template.meta.description.as_ref().unwrap()
    );
  }

  if !template.meta.extend.is_none() {
    let text = utils::vec_to_string(template.meta.extend.as_ref().unwrap());
    println!("extends: {}", text);
  }
}

pub fn display_repository(repository: &Repository) {
  println!("name: {}", repository.get_config().name);
  if repository.get_config().description.is_some() {
    println!(
      "description: {}",
      repository.get_config().description.to_owned().unwrap()
    );
  }
  // println!("path: {}", repository.directory.to_str().unwrap());
}

pub fn display_config(_config: &Config, config_path: &str) {
  let text = format!("Config loaded from: {}", config_path).green();
  println!("{}", text);
}

pub fn no_app_update() {
  let text = format!("There are no updates available").green();
  println!("{}", text);
}
