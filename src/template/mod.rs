use std::fs;
use std::fs::File;
use std::io::{Error, ErrorKind, Read, Write};
use std::path::Path;

mod meta;

use crate::config::Config;
use crate::renderer;
use crate::utils;

#[derive(Clone, Debug)]
pub struct Options {
  pub template: String,
  pub dir: String,
  pub name: String,
  pub repository: Option<String>,
  pub username: Option<String>,
  pub email: Option<String>,
  pub replace: bool,
}

pub struct Template {
  pub name: String,
  pub path: String,
}

pub fn copy(config: &Config, opts: Options) -> Result<(), Error> {
  // check if template exists and get absolute path
  let template_path = match get_template_path(config, &opts.template) {
    Ok(path) => path,
    Err(error) => {
      match error.kind() {
        ErrorKind::NotFound => renderer::errors::template_dir_not_found(&config.templates_dir),
        ErrorKind::PermissionDenied => {
          renderer::errors::template_dir_permission_denied(&config.templates_dir)
        }
        _ => renderer::errors::unknown(),
      }
      return Err(error);
    }
  };

  // load meta informations
  let meta = meta::load_meta(&template_path)?;

  // get list of all super templates
  let templates = match &meta.extend {
    None => Vec::new(),
    Some(x) => x.clone(),
  };

  // copy all super templates into the directory
  for template in templates {
    let mut opts = opts.clone();
    opts.template = template;

    copy(&config, opts)?;
  }

  // copy the template into the directory
  copy_template(config, &opts, &meta)?;

  Ok(())
}

pub fn get_all_templates(config: &Config) -> Result<Vec<Template>, Error> {
  let mut templates = Vec::<Template>::new();

  // check if folder exists
  match fs::read_dir(&config.templates_dir) {
    Ok(fc) => fc,
    Err(error) => return Err(error),
  };

  // Load meta of the templates directory
  let meta = meta::load_meta(&config.templates_dir)?;

  // Loop at all entries in templates directory
  for entry in fs::read_dir(&config.templates_dir).unwrap() {
    let entry = &entry.unwrap();
    // check if entry is file, if yes skip entry
    if !entry.path().is_dir() {
      continue;
    }

    let entry_path = entry.path().to_string_lossy().into_owned();
    let entry_meta = meta::load_meta(&entry_path)?;

    // If type is None or unqual template skip entry
    if entry_meta.kind.is_none() || entry_meta.kind != Some(String::from("template")) {
      continue;
    }

    let name;
    if entry_meta.name.is_none() {
      // if no name is provided use dir name
      name = entry
        .path()
        .file_name()
        .unwrap()
        .to_string_lossy()
        .into_owned();
    } else {
      name = entry_meta.name.unwrap();
    }

    if meta::exclude_file(&name, &meta) {
      continue;
    }

    // make all names lowercase
    let template = Template {
      name: utils::lowercase(&name),
      path: entry_path,
    };

    templates.push(template);
  }

  return Ok(templates);
}

fn copy_template(config: &Config, opts: &Options, meta: &meta::Meta) -> Result<(), Error> {
  // check if template exists
  let template_path = get_template_path(config, &opts.template)?;

  copy_folder(&template_path, &opts.dir, opts, meta)?;

  Ok(())
}

fn copy_folder(src: &str, target: &str, opts: &Options, meta: &meta::Meta) -> Result<(), Error> {
  // Loop at selected template directory
  for entry in fs::read_dir(src)? {
    let entry = &entry.unwrap();

    let source_path = &entry.path().to_string_lossy().into_owned();
    let source_name = &entry
      .path()
      .file_name()
      .unwrap()
      .to_string_lossy()
      .into_owned();
    let dir_path = target.to_string() + "/" + source_name;

    // check if entry if directory
    if entry.path().is_dir() {
      let dir = target.to_string() + "/" + source_name;
      match fs::create_dir(Path::new(&dir)) {
        Ok(()) => (),
        Err(error) => match error.kind() {
          ErrorKind::AlreadyExists => (),
          _ => return Err(error),
        },
      };

      copy_folder(&source_path, &dir, opts, meta)?
    } else {
      if meta::exclude_file(&source_name, meta) {
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

fn fill_template(template: &str, opts: &Options) -> Result<String, Error> {
  // replace placeholder with actual value
  let mut data = template.replace("{{name}}", &opts.name);

  if !opts.repository.is_none() {
    data = data.replace("{{repository}}", opts.repository.as_ref().unwrap());
  }

  if !opts.username.is_none() {
    data = data.replace("{{username}}", opts.username.as_ref().unwrap());
  }

  if !opts.email.is_none() {
    data = data.replace("{{email}}", opts.email.as_ref().unwrap());
  }

  Ok(data)
}

fn get_template_path(config: &Config, template: &str) -> Result<String, Error> {
  let templates = get_all_templates(config)?;

  // all templates are lowercase
  let template = utils::lowercase(template);

  let mut template_path: Option<String> = None;
  for temp in templates {
    if temp.name == template {
      template_path = Some(temp.path);
      break;
    }
  }

  if template_path.is_none() {
    renderer::errors::template_not_found(&template);
    return Err(std::io::Error::new(
      std::io::ErrorKind::NotFound,
      "Template not found",
    ));
  }

  let template_path = template_path.unwrap();

  // check if path exists
  fs::read_dir(&template_path)?;
  return Ok(template_path);
}
