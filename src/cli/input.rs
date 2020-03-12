use std::io;
use std::io::{Error};
use std::io::*;

pub fn get_value(name: &str, required: bool, default: Option<&str>) -> std::result::Result<Option<String>, Error> {
  let mut message;
  
  // check if required, add question mark for optional parameters
  if required {
    message = format!("Enter {}: ", name);
  } else {
    message = format!{"Enter {}?: ", name};
  }

  // check if default values is provided
  if !default.is_none() {
    message = format!("message ({})", default.unwrap());
  }

  // finally print message
  print!("{}", message);

  // directly print message
  io::stdout().flush()?;

  let mut value = String::new();
  loop {
    io::stdin().read_line(&mut value).expect("error: unable to read user input");
    // remove whitespaces and new line
    value = value.trim().to_string();
    if value == "" && !required {
      return Ok(None);
    } else if value != "" {
      break;
    }
  }

  Ok(Some(value))
}