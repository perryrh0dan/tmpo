use std::fmt;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{Error, Read};
use std::path::Path;

use crate::error::RunError;
use crate::git;

extern crate serde;
use serde::{de, Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub enum Type {
  #[serde(alias = "repository")]
  REPOSITORY,
  #[serde(alias = "template")]
  TEMPLATE,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub enum TemplateType {
  #[serde(alias = "project")]
  PROJECT,
  #[serde(alias = "snippet")]
  SNIPPET,
}

impl TemplateType {
  pub fn default() -> Self { TemplateType::PROJECT }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RepositoryMeta {
  #[serde(alias = "kind")]
  #[serde(rename(serialize = "type", deserialize = "type"))]
  pub kind: Type,
  pub name: String,
  pub version: Option<String>,
  pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TemplateMeta {
  #[serde(alias = "kind")]
  #[serde(rename(serialize = "type", deserialize = "type"))]
  pub kind: Type,
  #[serde(rename(serialize = "subType", deserialize = "subType"))]
  #[serde(default = "TemplateType::default")]
  pub sub_type: TemplateType,
  pub name: String,
  pub version: Option<String>,
  pub description: Option<String>,
  pub visible: Option<bool>,
  pub scripts: Option<Scripts>,
  pub extend: Option<Vec<String>>,
  pub exclude: Option<Vec<String>>,
  pub renderer: Option<Renderer>,
  pub info: Option<String>,
}

impl fmt::Display for Type {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match *self {
      Type::REPOSITORY => write!(f, "Repository"),
      Type::TEMPLATE => write!(f, "Template"),
    }
  }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Renderer {
  pub exclude: Option<Vec<String>>,
  pub values: Option<ValuesWrapper>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Value {
  pub key: String,
  pub label: Option<String>,
  pub default: Option<String>,
  pub required: Option<bool>,
}

impl PartialEq for Value {
  fn eq(&self, other: &Self) -> bool {
    self.key == other.key
  }
}

impl Eq for Value {}

impl Hash for Value {
  fn hash<H: Hasher>(&self, state: &mut H) {
      self.key.hash(state);
  }
}

impl Value {
  pub fn get_label(&self) -> String {
    if self.label.is_some() {
      return self.label.clone().unwrap()
    } else {
      return self.key.clone()
    }
  }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum ValuesWrapper {
  Values(Vec<Value>),
  StringArray(Vec<String>)
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Scripts {
  pub before_install: Option<String>,
  pub after_install: Option<String>,
}

pub fn load<T: de::DeserializeOwned>(dir: &Path) -> Result<T, Error> {
  let meta_path = dir.join("meta.json");

  // Open file
  let mut src = File::open(Path::new(&meta_path))?;
  let mut data = String::new();

  // Write to data string
  src.read_to_string(&mut data)?;
  let meta: T = serde_json::from_str(&data)?;

  Ok(meta)
}

pub fn fetch<T: de::DeserializeOwned>(options: &git::Options) -> Result<T, RunError> {
  let provider = if options.provider.is_some() {
    options.provider.clone().unwrap()
  } else {
    return Err(RunError::Meta(String::from("No provider was provided")));
  };

  let meta = match provider {
    git::Provider::GITHUB => git::github::fetch_meta::<T>(options)?,
    git::Provider::GITLAB => git::gitlab::fetch_meta::<T>(options)?,
  };

  Ok(meta)
}

impl RepositoryMeta {
  pub fn new(kind: Type) -> RepositoryMeta {
    RepositoryMeta {
      kind: kind,
      name: String::from(""),
      version: Some(String::from("1.0.0")),
      description: Some(String::from("")),
    }
  }
}

impl TemplateMeta {
  pub fn new(kind: Type) -> TemplateMeta {
    TemplateMeta {
      kind: kind,
      sub_type: TemplateType::PROJECT,
      name: String::from(""),
      version: Some(String::from("1.0.0")),
      description: Some(String::from("")),
      visible: Some(true),
      scripts: Some(Scripts {
        before_install: None,
        after_install: None,
      }),
      extend: None,
      exclude: None,
      renderer: Some(Renderer {
        exclude: None,
        values: None,
      }),
      info: None,
    }
  }

  pub fn get_values(&self) -> Vec<Value> {
    let renderer = match self.renderer.to_owned() {
      Some(data) => data,
      None => return vec![],
    };

    let generic_values = match renderer.values {
      Some(x) => x, //TODO
      None => return vec![],
    };

    let values = match generic_values {
      ValuesWrapper::Values(v) => v,
      ValuesWrapper::StringArray(v) => {
        let mut values = vec![];
        for value in v {
          values.push(Value{
            key: value,
            label: None,
            default: None,
            required: None,
          })
        }
        return values
      },
    };

    return values
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn deserialize_template_values_old() {
    let data = r#"[
      "value1", "value2"
    ]"#;

    let result = vec![
      Value {
        key: String::from("value1"),
        label: None,
        default: None,
        required: None,
      },
      Value {
        key: String::from("value2"),
        label: None,
        default: None,
        required: None,
      },
    ];

    let meta: ValuesWrapper = serde_json::from_str(&data).unwrap();
    let values = match meta {
      ValuesWrapper::Values(_values) => panic!("wrong deserialization type"),
      ValuesWrapper::StringArray(values) => values,
    };

    assert_eq!(values.len(), result.len());
  }

  #[test]
  fn deserialize_template_values_new() {
    let data = r#"[
      {
        "key": "value1"
      },
      {
        "key": "value2"
      }
    ]"#;

    let result = vec![
      Value {
        key: String::from("value1"),
        label: None,
        default: None,
        required: None,
      },
      Value {
        key: String::from("value2"),
        label: None,
        default: None,
        required: None,
      },
    ];

    let generic_values: ValuesWrapper = serde_json::from_str(&data).unwrap();
    let values = match generic_values {
      ValuesWrapper::Values(values) => values,
      ValuesWrapper::StringArray(_) => panic!("wrong deserialization type"),
    };

    assert_eq!(values.len(), result.len());
  }
}
