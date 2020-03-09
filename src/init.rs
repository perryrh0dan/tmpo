use std::fs;
use fs_extra::dir;

pub fn initProject(name: String, path: String) {
  //Create directory  
  let result = fs::create_dir(path);

  //Copy files
  let options = dir::CopyOptions::new(); //Initialize default values for CopyOptions
  dir::copy("C:/Dev/init/templates/default", path, &options);
}
