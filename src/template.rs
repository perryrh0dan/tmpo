use std::fs;
use std::fs::File;
use std::io::{Read, Write, Error};
use std::path::Path;
use serde::{Serialize, Deserialize};

use crate::config::Config; 

#[derive(Serialize, Deserialize, Debug)]
pub struct TemplateMeta {
  pub name: Option<String>,
  pub extend: Option<Vec<String>>,
  pub exclude: Option<Vec<String>>,
}

#[derive(Clone, Debug)]
pub struct CopyOpts {
  pub template: String,
  pub target: String,
  pub name: String,
  pub repository: Option<String>,
}

pub fn copy(config: &Config, opts: CopyOpts) -> Result<(), Error> {
  // check if template exists
  let template_path = get_template_path(config, &opts.template)?;

  let meta = load_meta(&template_path)?;
  let templates = match meta.extend {
    None => Vec::new(),
    Some(x) => x,
  };


  for template in templates {
    let mut opts = opts.clone();
    opts.template = template;

    copy(&config, opts)?;
  }

  copy_template(config, &opts)?;

  Ok(())
}

pub fn get_all_templates(config: &Config) -> Result<Vec<String>, Error> {
  let mut templates = Vec::new();
  
  // check if folder exists
  match fs::read_dir(&config.templates_dir) {
    Ok(fc) => fc,
    Err(error) => return Err(error),
  };

  // Load meta of the templates directory
  let meta = load_meta(&config.templates_dir)?;
  let items = match meta.exclude {
    None => Vec::<String>::new(),
    Some(x) => x
  };

  // Loop at all entries in templates directory
  for entry in fs::read_dir(&config.templates_dir).unwrap() {
    let entry = &entry.unwrap();
    let name = entry.path().file_name().unwrap().to_string_lossy().into_owned();

    if name == "meta.json" {
      continue;
    };

    // check meta exclude
    let mut exclude = false;
    for item in items.iter() {
      if item == &name {
        exclude = true;
      }

      if exclude {
        break;
      }
    };

    if exclude {
      continue;
    }

    templates.push(name.to_string());
  }

  return Ok(templates);
}

fn copy_template(config: &Config, opts: &CopyOpts) -> Result<(), Error> {
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
    
    data = fill_template(&data, &opts)?;

    // create file
    let mut dst = File::create(target_path)?;
    dst.write(data.as_bytes())?;
  }

  Ok(())
}

fn fill_template(tempalte: &str, opts: &CopyOpts) -> Result<String, Error> {
  // replace placeholder with actual value
  let mut data = tempalte.replace("{{name}}", &opts.name);

  if !opts.repository.is_none() {
    data = data.replace("{{repository}}", opts.repository.as_ref().unwrap());
  }

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
    let meta = TemplateMeta{ name: None, extend: None, exclude: None };
    return Ok(meta);
  }

  // Open file
  let mut src = File::open(Path::new(&dir))?;
  let mut data = String::new();

  // Write to data string
  src.read_to_string(&mut data)?;
  let meta: TemplateMeta = serde_json::from_str(&data)?;
  Ok(meta)
}