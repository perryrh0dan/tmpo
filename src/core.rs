use std::fs;
use std::io::{Error};
use std::path::Path;

use crate::config::Config; 
use crate::template;
use crate::renderer;

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
  let templates = template::get_all_templates(config)?;

  renderer::list_templates(&templates);

  Ok(())
}