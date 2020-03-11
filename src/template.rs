
use std::io::{Error};
pub fn replace_placeholders(data: &str, name: &str) -> Result<String, Error> {
  // replace placeholder with actual value
  let data = data.replace("{{name}}", name);

  Ok(data)
}