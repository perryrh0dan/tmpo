use std::fs;
use std::fs::File;
use std::io::{Error, Read, Write};
use std::path::Path;
use std::path::PathBuf;

use crate::git;

extern crate dirs;
extern crate serde;
extern crate serde_yaml;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Config {
  pub templates_dir: String,
  pub templates_repositories: Vec<git::RepoOptions>,
}

pub fn init(_verbose: bool) -> Result<Config, Error> {
  let dir = match dirs::home_dir() {
    Some(path) => PathBuf::from(path),
    None => PathBuf::from(""),
  };

  let mut dir = dir.into_os_string().into_string().unwrap();
  dir = dir + "/.charon";

  ensure_root_dir(&dir)?;
  ensure_config_file(&dir)?;
  let config = load_config(&dir)?;
  Ok(config)
}

fn ensure_root_dir(dir: &str) -> Result<(), Error> {
  let r = fs::create_dir_all(Path::new(dir));
  match r {
    Ok(fc) => fc,
    Err(error) => return Err(error),
  };

  Ok(())
}

fn ensure_config_file(dir: &str) -> Result<(), Error> {
  let conf_path = String::from(dir) + "/config.yaml";
  if Path::new(&conf_path).exists() {
    return Ok(());
  }

  let default_config = get_default_config(dir);
  let new_data = serde_yaml::to_string(&default_config).unwrap();
  let mut dst = File::create(conf_path)?;
  dst.write(new_data.as_bytes())?;

  Ok(())
}

fn load_config(dir: &str) -> Result<Config, Error> {
  let dir = String::from(dir) + "/config.yaml";
  // Open file
  let mut src = File::open(Path::new(&dir))?;
  let mut data = String::new();

  // Write to data string
  src.read_to_string(&mut data)?;
  let config: Config = serde_yaml::from_str(&data).unwrap();
  return Ok(config);
}

fn get_default_config(dir: &str) -> Config {
  let template_dir = format!("{}{}", dir, "/templates");
  let mut repo_options = Vec::<git::RepoOptions>::new();

  repo_options.push(
    git::RepoOptions {
      enabled: true,
      url: String::from("https://github.com/perryrh0dan/templates"),
      auth: String::from("none"),
      token: None,
      username: None,
      password: None,
    },
  );

  let config = Config {
    templates_dir: template_dir,
    templates_repositories: repo_options
  };

  return config;
}

impl Config {
  pub fn get_repository_config(&self, url: &str) -> Option<git::RepoOptions> {
    // let result = base64::decode_config(url, base64::URL_SAFE).unwrap();
    // let url = String::from_utf8_lossy(&result);

    let config = self.templates_repositories.iter().find(| &x | {
      x.url == url
    });

    if config.is_some() {
      return Some(config.unwrap().clone())
    } else {
      return None;
    }
  }
}
