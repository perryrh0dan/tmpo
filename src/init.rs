use std::fs;
use std::fs::File;
use std::path::Path;
use std::io::{self, Read, Write, Error};
use fs_extra::dir;

const DefaultTemplate: &str = "/home/thomas/dev/init/templates/default";

pub fn init_project(name: String, path: String) -> Result<(), Error> {
  //Create directory  
  let full_path = path + "/" + &name;
  let r = fs::create_dir(Path::new(&full_path));
  let r = match r {
    Ok(fc) => fc,
    Err(error) => return Err(error),
  };

  //Copy files
  let options = dir::CopyOptions::new(); //Initialize default values for CopyOptions

  // Loop at default templates directory
  for entry in fs::read_dir(DefaultTemplate).unwrap() {

    let entry = &entry.unwrap();
    let source_file_path = &entry.path().to_string_lossy().into_owned();
    let source_file_name = &entry.path().file_name().unwrap().to_string_lossy().into_owned();

    let target_file_path = full_path.to_string() + "/" + source_file_name;
    replace_placeholders(&source_file_path, &target_file_path, &name)?;
  }

  Ok(())
}

fn replace_placeholders(src: &str, target: &str, name: &str) -> Result<(), io::Error> {
  // Open file
  let mut src = File::open(Path::new(src))?;
  let mut data = String::new();

  // Write to data string
  src.read_to_string(&mut data);

  // close file
  drop(src);

  // replace placeholder with actual value
  let new_data = data.replace("{{name}}", name);

  // create file
  let mut dst = File::create(target)?;
  dst.write(new_data.as_bytes())?;

  Ok(())
}
