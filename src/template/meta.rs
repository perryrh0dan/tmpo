use std::fs::File;
use std::io::{Read, Error};
use std::path::Path;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Meta {
  pub kind: Option<String>,
  pub name: Option<String>,
  pub extend: Option<Vec<String>>,
  pub exclude: Option<Vec<String>>,
}

pub fn load_meta(dir: &str) -> Result<Meta, Error> {
  let meta_path = String::from(dir) + "/meta.json";
  // check if file exists
  if !Path::new(&meta_path).exists() {
    let meta = Meta { 
      kind: None,
      name: None, 
      extend: None, 
      exclude: None };
    return Ok(meta);
  }

  // Open file
  let mut src = File::open(Path::new(&meta_path))?;
  let mut data = String::new();

  // Write to data string
  src.read_to_string(&mut data)?;
  let meta: Meta = serde_json::from_str(&data)?;
  Ok(meta)
}

pub fn exclude_file(name: &str, meta: &Meta) -> bool {
  if name == "meta.json" {
    return true;
  };
  
  let items = match &meta.exclude {
    None => return false,
    Some(x) => x
  };

  // check meta exclude
  for item in items.iter() {
    if item == &name {
      return true
    }
  };

  return false
}