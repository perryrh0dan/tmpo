use std::fs;
use std::fs::File;
use std::io::{Read, Write, Error};
use std::path::Path;
use serde::{Serialize, Deserialize};

use crate::config::Config; 

#[derive(Serialize, Deserialize, Debug)]
pub struct TemplateMeta {
  pub name: String,
  pub extends: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct CopyOpts {
  pub template: String,
  pub name: String,
  pub target: String,
}

pub fn copy(config: &Config, opts: CopyOpts) -> Result<(), Error> {
  // check if template exists
  let template_path = get_template_path(config, &opts.template)?;

  let meta = load_meta(&template_path)?;

  for template in meta.extends {
    let mut opts = opts.clone();
    opts.template = template;

    // copy_template(config, &opts)?;
    copy(&config, opts)?;
  }

  copy_template(config, &opts)?;

  Ok(())
}

pub fn copy_template(config: &Config, opts: &CopyOpts) -> Result<(), Error> {
  // check if template exists
  let template_path = get_template_path(config, &opts.template)?;

  // Loop at selected template directory
  for entry in fs::read_dir(template_path)? {
    let entry = &entry.unwrap();

    let source_path = &entry.path().to_string_lossy().into_owned();
    let source_name = &entry.path().file_name().unwrap().to_string_lossy().into_owned();

    if source_name == "meta.json" {
      continue;
    }

    let target_path = opts.target.to_string() + "/" + source_name;

    // Open file
    let mut src = File::open(Path::new(&source_path))?;
    let mut data = String::new();

    // Write to data string
    src.read_to_string(&mut data)?;

    // close file
    drop(src);
    
    data = replace_placeholders(&data, &opts.name)?;

    // create file
    let mut dst = File::create(target_path)?;
    dst.write(data.as_bytes())?;
  }

  Ok(())
}

fn replace_placeholders(data: &str, name: &str) -> Result<String, Error> {
  // replace placeholder with actual value
  let data = data.replace("{{name}}", name);

  Ok(data)
}

fn get_template_path(config: &Config, template: &str) -> Result<String, Error> {
  let template_path = config.templates_dir.clone() + "/" + template;

  fs::read_dir(&template_path)?;  
  return Ok(template_path);
}

fn load_meta(template_path: &str) -> Result<TemplateMeta, Error> {
  let dir = String::from(template_path) + "/meta.json";
  // check if file exists
  if !Path::new(&dir).exists() {
    let meta = TemplateMeta{ name: "".to_string(), extends: Vec::new() };
    return Ok(meta);
  }

  // Open file
  let mut src = File::open(Path::new(&dir))?;
  let mut data = String::new();

  // Write to data string
  src.read_to_string(&mut data)?;
  let meta: TemplateMeta = serde_json::from_str(&data).unwrap();
  Ok(meta)
}