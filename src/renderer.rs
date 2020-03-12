use std::io::{Error};

pub fn list_templates(templates: &Vec<String>) -> Result<(), Error> {
  for template in templates {
    println!("{}", template);
  };

  return Ok(());
} 