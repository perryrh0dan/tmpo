use colored::Colorize;

pub fn list_templates(templates: &Vec<String>) {
  for template in templates {
    println!("{}", template);
  };
} 

pub fn success_create() {
  let text = format!("{}", "Created project".green());
  println!("{}", text);
}

pub fn check_template_updates() {
  let text = format!("{}", "Check for template updates".green());
  println!("{}", text);
}

pub fn success_update_templates() {
  let text = format!("{}", "Successful updated templates".green());
  println!("{}", text);
}

pub fn error_update_templates() {
  let text = format!("{}", "Error updating templates".red());
  println!("{}", text);
}