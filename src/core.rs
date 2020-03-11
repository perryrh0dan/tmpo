use std::fs;
use std::io::{Error};
use std::path::Path;

use crate::config::Config; 
use crate::template;

pub struct InitOpts {
  pub name: String,
  pub template: String,
  pub dir: String,
}

pub fn init(config: &Config, opts: InitOpts) -> Result<(), Error> {
  //Create directory  
  let target = opts.dir + "/" + &opts.name;
  let r = fs::create_dir(Path::new(&target));
  match r {
    Ok(fc) => fc,
    Err(error) => return Err(error),
  };

  let copy_opts = template::CopyOpts {
    template: opts.template.to_string(),
    name: opts.name.to_string(),
    target: target,
  };
  template::copy(config, copy_opts)?;

  Ok(())
}

pub fn list(config: &Config) -> Result<(), Error> {
  let mut templates = Vec::new();

  // check if folder exists
  match fs::read_dir(&config.templates_dir) {
    Ok(fc) => fc,
    Err(error) => return Err(error),
  };

  for entry in fs::read_dir(&config.templates_dir).unwrap() {
    let template = &entry.unwrap();
    templates.push(template.path().file_name().unwrap().to_string_lossy().into_owned())
  }

  for template in templates {
    println!("{}", template)
  }

  Ok(())
}