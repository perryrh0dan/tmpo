use crate::config::Config;
use crate::template;
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

pub fn display_config(_config: &Config, config_path: &str) {
    let text = format!("Config loaded from: {}", config_path).green();
    println!("{}", text);
}

pub fn no_app_update() {
    let text = format!("tmpo is already up to date").green();
    println!("{}", text);
}

pub fn app_update_available() {
    let text = format!("Update available").green();
    println!("{}", text);
}
