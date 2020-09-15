use std::fs::File;
use std::io::{Error, Read};
use std::path::Path;

use serde::{Deserialize, Serialize};
use reqwest;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Meta {
    pub kind: String,
    pub name: String,
    pub version: Option<String>,
    pub description: Option<String>,
    pub scripts: Option<Scripts>,
    pub extend: Option<Vec<String>>,
    pub exclude: Option<Vec<String>>,
    pub renderer: Option<Renderer>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Renderer {
    pub exclude: Option<Vec<String>>,
    pub values: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Scripts {
    pub before_install: Option<String>,
    pub after_install: Option<String>,
}

pub fn load(dir: &Path) -> Result<Meta, Error> {
    let meta_path = dir.join("meta.json");

    // check if file exists
    if !meta_path.exists() {
        let meta = Meta::new();
        return Ok(meta);
    }

    // Open file
    let mut src = File::open(Path::new(&meta_path))?;
    let mut data = String::new();

    // Write to data string
    src.read_to_string(&mut data)?;
    let meta: Meta = serde_json::from_str(&data)?;
    Ok(meta)
}

// pub fn fetch(remote_url: &str) -> Result<Meta, Error> {
//   // TODO try to fetch meta data from url

//   // https://github.com/perryrh0dan/templates/blob/master/meta.json
//   let meta_url = format!("{}/blob/master/meta.json", remote_url);
//   let response = reqwest::get()

// }

impl Meta {
  pub fn new() -> Meta {
    Meta {
      kind: String::from(""),
      name: String::from(""),
      version: None,
      description: None,
      scripts: Some(Scripts {
          before_install: None,
          after_install: None,
      }),
      extend: None,
      exclude: None,
      renderer: Some(
        Renderer {
          exclude: None,
          values: None,
        }
      ),
    }
  }

  pub fn get_values(&self) -> Vec<String> {
    let renderer = match self.renderer.to_owned() {
      Some(data) => data,
      None => return vec!{},
    };

    match renderer.values {
      Some(x) => x,
      None => return vec!{},
    }
  }

  pub fn get_before_install_script(&self) -> Option<String> {
    if self.scripts.is_some() {
      let scripts = self.scripts.as_ref().unwrap();
      if scripts.before_install.is_some() {
        let script = scripts.before_install.to_owned().unwrap();

        return Some(script);
      }
    }

    return None;
  }

  pub fn get_after_install_script(&self) -> Option<String> {
    if self.scripts.is_some() {
      let scripts = self.scripts.as_ref().unwrap();
      if scripts.after_install.is_some() {
        let script = scripts.after_install.to_owned().unwrap();

        return Some(script);
      }
    }

    return None;
  }
}
