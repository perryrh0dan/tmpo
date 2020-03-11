use std::fs;
use std::fs::File;
use std::path::Path;
use std::io::{Read, Write, Error};

use crate::config::Config; 
use crate::template;

pub struct InitOpts {
  pub name: String,
  pub template: String,
  pub dir: String,
}

pub fn init(config: &Config, opts: InitOpts) -> Result<(), Error> {
  //Create directory  
  let full_path = opts.dir + "/" + &opts.name;
  let r = fs::create_dir(Path::new(&full_path));
  match r {
    Ok(fc) => fc,
    Err(error) => return Err(error),
  };


  // Todo check if template exists
  let template_dir = config.templates_dir.clone() + "/" + &opts.template;

  // Loop at selected template directory
  for entry in fs::read_dir(template_dir).unwrap() {
    let entry = &entry.unwrap();

    let source_file_path = &entry.path().to_string_lossy().into_owned();
    let source_file_name = &entry.path().file_name().unwrap().to_string_lossy().into_owned();
    let target_file_path = full_path.to_string() + "/" + source_file_name;

    // Open file
    let mut src = File::open(Path::new(&source_file_path))?;
    let mut data = String::new();

    // Write to data string
    src.read_to_string(&mut data)?;

    // close file
    drop(src);
    
    data = template::replace_placeholders(&data, &opts.name)?;
  
    // create file
    let mut dst = File::create(target_file_path)?;
    dst.write(data.as_bytes())?;
  }

  Ok(())
}

pub fn list(config: &Config) -> Result<(), Error> {
  let mut templates = Vec::new();

  for entry in fs::read_dir(&config.templates_dir).unwrap() {
    let template = &entry.unwrap();
    templates.push(template.path().file_name().unwrap().to_string_lossy().into_owned())
  }

  for template in templates {
    println!("{}", template)
  }

  Ok(())
}