use std::fs;
use std::io::{Error};
use std::path::{Path};

use crate::config::Config;
use crate::git; 
use crate::template;
use crate::renderer;

pub struct InitOpts {
  pub name: String,
  pub template: String,
  pub dir: String,
  pub repository: Option<String>,
}

pub fn init(config: &Config, opts: InitOpts) -> Result<(), Error> {
  //Create directory  
  let dir = opts.dir + "/" + &opts.name;
  let r = fs::create_dir(Path::new(&dir));
  match r {
    Ok(fc) => fc,
    Err(error) => return Err(error),
  };

  let copy_opts = template::CopyOpts {
    template: String::from(&opts.template),
    dir: String::from(&dir),
    name: String::from(&opts.name),
    repository: opts.repository.clone(),
  };

  template::copy(config, copy_opts)?;

  if !opts.repository.is_none() {
    match git::init(&dir, &opts.repository.unwrap()) {
      Ok(()) => (),
      Err(_e) => renderer::error_init_repository(),
    }
  }

  renderer::success_create();

  Ok(())
}

pub fn list(config: &Config) -> Result<(), Error> {
  let templates = template::get_all_templates(config)?;

  renderer::list_templates(&templates);

  Ok(())
}