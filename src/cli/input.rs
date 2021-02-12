use crate::context::Context;
use crate::error::RunError;
use crate::utils;

extern crate dialoguer;
use dialoguer::{theme::ColorfulTheme, Input, Password, Select};

pub fn text(text: &str, allow_empty: bool) -> Result<String, RunError> {
  match Input::<String>::with_theme(&ColorfulTheme::default())
    .with_prompt(text)
    .allow_empty(allow_empty)
    .interact()
  {
    Ok(value) => Ok(value),
    Err(error) => Err(RunError::IO(error)),
  }
}

pub fn text_with_default(ctx: &Context, text: &str, default: &str) -> Result<String, RunError> {
  if !ctx.yes {
    let input = match Input::<String>::with_theme(&ColorfulTheme::default())
      .with_prompt(text)
      .allow_empty(true)
      .default(String::from(default))
      .show_default(true)
      .interact()
    {
      Ok(value) => value,
      Err(error) => return Err(RunError::IO(error)),
    };

    if input == "" {
      return Ok(String::from(default));
    }

    return Ok(input);
  } else {
    return Ok(String::from(default));
  }
}

pub fn confirm(text: &str) -> bool {
  let mut question = text.to_owned();
  question.push_str(" [Y/n]");

  match Input::<String>::with_theme(&ColorfulTheme::default())
    .with_prompt(&question)
    .allow_empty(false)
    .interact()
  {
    Ok(value) => value == "Y" || value == "y",
    Err(_error) => false,
  }
}

pub fn password(text: &str) -> Result<String, RunError> {
  match Password::with_theme(&ColorfulTheme::default())
    .with_prompt(text)
    .interact()
  {
    Ok(value) => Ok(value),
    Err(error) => Err(RunError::IO(error)),
  }
}

pub fn select(name: &str, options: &Vec<String>) -> Result<String, RunError> {
  if options.len() == 0 {
    return Err(RunError::Input(String::from("No Options")));
  };

  // TODO
  // if options.len() == 1 {
  //    return Ok(options[0]);
  // }

  // capitalize options
  let mut capitalized_options = Vec::new();

  for value in options {
    capitalized_options.push(utils::capitalize(value));
  }

  // sort options
  capitalized_options.sort();

  let selection = match Select::with_theme(&ColorfulTheme::default())
    .with_prompt(String::from("Select a ") + name)
    .default(0)
    .items(&capitalized_options)
    .interact()
  {
    Ok(selection) => selection,
    Err(error) => return Err(RunError::IO(error)),
  };

  let result = utils::lowercase(&capitalized_options[selection]);
  Ok(result)
}
