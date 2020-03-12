use std::io;
use std::io::{Error};
use std::io::*;

pub fn get_value(name: &str, required: bool, default: Option<&str>) -> std::result::Result<Option<String>, Error> {
  if required {
    print!("Enter {}: ", name);
  } else {
    print!{"Enter {}?: ", name};
  }
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