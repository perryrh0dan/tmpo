use std::io::{Error, ErrorKind};

use dialoguer::{theme::ColorfulTheme, Input, Select};

pub fn input(text: &str, allow_empty: bool ) -> Option<String> {
  match Input::<String>::with_theme(&ColorfulTheme::default())
    .with_prompt(text)
    .allow_empty(allow_empty)
    .interact()
  {
    Ok(value) => Some(value),
    Err(_error) => return None,
  }
}

pub fn select(name: &str, options: &Vec<String>) -> Result<String, Error> {
  if options.len() == 0 {
    return Err(Error::from(ErrorKind::InvalidData))
  };

  let selection = match Select::with_theme(&ColorfulTheme::default())
    .with_prompt(String::from("Select a ") + name)
    .default(0)
    .items(options)
    .interact()
  {
    Ok(selection) => selection,
    Err(_error) => return Err(Error::from(ErrorKind::Interrupted)),
  };
  let result = String::from(&options[selection]);
  return Ok(result);
}
