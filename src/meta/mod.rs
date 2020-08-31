use std::fs::File;
use std::io::{Error, Read};
use std::path::Path;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Meta {
    pub kind: Option<String>,
    pub name: Option<String>,
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
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Scripts {
    pub before_install: Option<String>,
    pub after_install: Option<String>,
}

pub fn load_meta(dir: &str) -> Result<Meta, Error> {
    let meta_path = String::from(dir) + "/meta.json";

    // check if file exists
    if !Path::new(&meta_path).exists() {
        let meta = default();
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

pub fn default() -> Meta {
    let meta = Meta {
        kind: None,
        name: None,
        version: None,
        description: None,
        scripts: Some(Scripts {
            before_install: None,
            after_install: None,
        }),
        extend: None,
        exclude: None,
        renderer: Some(Renderer {
            exclude: None,
        }),
    };

    return meta;
}
