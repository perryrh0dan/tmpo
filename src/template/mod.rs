use std::fs;
use std::fs::File;
use std::io::{Read, Write, Error, ErrorKind};
use std::path::Path;
use serde::{Serialize, Deserialize};

use crate::config::Config; 
use crate::renderer;

#[derive(Serialize, Deserialize, Debug)]
pub struct TemplateMeta {
  pub name: Option<String>,
  pub extend: Option<Vec<String>>,
  pub exclude: Option<Vec<String>>,
}

#[derive(Clone, Debug)]
pub struct CopyOpts {
  pub template: String,
  pub dir: String,
  pub name: String,
  pub repository: Option<String>,
}

pub fn copy(config: &Config, opts: CopyOpts) -> Result<(), Error> {
  // check if template exists
  let template_path = match get_template_path(config, &opts.template) {
    Ok(path) => path,
    Err(error) => {
      match error.kind() {
        ErrorKind::NotFound => renderer::errors::template_dir_not_found(&config.templates_dir),
        ErrorKind::PermissionDenied => renderer::errors::template_dir_permission_denied(&config.templates_dir),
        _ => renderer::errors::unknown(),
      }
      return Err(error);
    }
  };

  let meta = load_meta(&template_path)?;
  let templates = match &meta.extend {
    None => Vec::new(),
    Some(x) => x.clone(),
  };


  for template in templates {
    let mut opts = opts.clone();
    opts.template = template;

    copy(&config, opts)?;
  }

  copy_template(config, &opts, &meta)?;

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

  // Loop at all entries in templates directory
  for entry in fs::read_dir(&config.templates_dir).unwrap() {
    let entry = &entry.unwrap();
    let name = entry.path().file_name().unwrap().to_string_lossy().into_owned();

    if meta_exclude(&name, &meta){
      continue;
    }

    templates.push(name.to_string());
  }

  return Ok(templates);
}

fn copy_template(config: &Config, opts: &CopyOpts, meta: &TemplateMeta) -> Result<(), Error> {
  // check if template exists
  let template_path = get_template_path(config, &opts.template)?;

  copy_folder(&template_path, &opts.dir, opts, meta)?;

  Ok(())
}

fn copy_folder(src: &str, target: &str, opts: &CopyOpts, meta: &TemplateMeta) -> Result<(), Error> {
  // Loop at selected template directory
  for entry in fs::read_dir(src)? {
    let entry = &entry.unwrap();

    let source_path = &entry.path().to_string_lossy().into_owned();
    let source_name = &entry.path().file_name().unwrap().to_string_lossy().into_owned();
    let dir_path = target.to_string() + "/" + source_name;

    // check if entry if directory
    if entry.path().is_dir() {
      let dir = target.to_string() + "/" + source_name;
      fs::create_dir(Path::new(&dir))?;

      copy_folder(&source_path, &dir, opts, meta)?
    } else {
      if meta_exclude(&source_name, meta) {
        continue;
      }

      // Open file
      let mut src = File::open(Path::new(&source_path))?;
      let mut data = String::new();

      // Write to data string
      src.read_to_string(&mut data)?;

      // close file
      drop(src);
      
      data = fill_template(&data, &opts)?;

      // create file
      let mut dst = File::create(dir_path)?;
      dst.write(data.as_bytes())?;
    }
  }

  Ok(())
}

fn meta_exclude(name: &str, meta: &TemplateMeta) -> bool {
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