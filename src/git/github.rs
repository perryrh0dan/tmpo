use std::ops::Add;

use crate::error::RunError;
use crate::git;
use crate::meta::Meta;

extern crate reqwest;
extern crate regex;
use regex::Regex;

pub fn fetch_meta(options: &git::Options) -> Result<Meta, RunError> {
  let response = match fetch(options) {
    Ok(resp) => resp,
    Err(error) => return Err(error),
  };

  let meta: Meta = match response.json() {
    Ok(data) => data,
    Err(error) => return Err(RunError::Meta(format!("{}", error))),
  };

  Ok(meta)
}

fn fetch(options: &git::Options) -> Result<reqwest::blocking::Response, RunError> {
  // URL must bet provided
  let url = if options.url.is_some() {
    options.url.clone().unwrap()
  } else {
    return Err(RunError::Meta(String::from("No url was provided")));
  };

  // Provider must be provided
  let provider = if options.provider.is_some() {
    options.provider.clone().unwrap()
  } else {
    return Err(RunError::Meta(String::from("No provider was provided")));
  };

  // Only Auth Type TOKEN and NONE is supported
  let auth = if options.auth.is_some() {
    options.auth.clone().unwrap()
  } else {
    return Err(RunError::Meta(String::from("No Auth type was provided")));
  };

  if provider != git::Provider::GITHUB {
    return Err(RunError::Meta(String::from("Provider not supported")));
  }

  if auth != git::AuthType::BASIC && auth != git::AuthType::TOKEN {
    return Err(RunError::Meta(String::from("Auth type is not supported for fetching")));
  }

  let meta_url = match build_meta_url(&url) {
    Ok(value) => value,
    Err(error) => return Err(error),
  };

  let mut headers = reqwest::header::HeaderMap::new();
  headers.insert(
    reqwest::header::ACCEPT,
    "application/json".parse().unwrap(),
  );

  if auth == git::AuthType::TOKEN && options.token.is_some() {
    headers.insert(
      "Authorization",
      format!("token {}", options.token.clone().unwrap()).parse().unwrap(),
    );
  }

  let client = reqwest::blocking::Client::new();
  let response = match client.get(&meta_url).headers(headers).send() {
    Ok(resp) => resp,
    Err(error) => return Err(RunError::Meta(format!("{}", error))),
  };

  return Ok(response)
}

pub fn build_meta_url(repository_url: &str) -> Result<String, RunError> {
  // https://raw.githubusercontent.com/perryrh0dan/templates/master/meta.json
  let re = Regex::new(".+?://github.com").unwrap();
  let partial_url = re.replace(repository_url, "https://raw.githubusercontent.com").to_owned();
  let meta_url = partial_url.to_string().add("/master/meta.json");

  Ok(meta_url)
}

#[test]
fn build_meta_url_success() {
  let repository_url = "https://github.com/perryrh0dan/templates";

  let url = build_meta_url(repository_url);
  assert_eq!(url.unwrap(), "https://raw.githubusercontent.com/perryrh0dan/templates/master/meta.json");
}

#[test]
fn build_meta_url_failure() {
  let repository_url = "https://github.de/perryrh0dan/templates";

  let url = build_meta_url(repository_url);
  assert_eq!(url.unwrap(), "test");
}

#[test]
fn fetch_meta_success() {
  let mut options = git::Options::new();
  options.enabled = true;
  options.provider = Some(git::Provider::GITHUB);
  options.auth = Some(git::AuthType::NONE);
  options.url = Some(String::from("https://github.com/perryrh0dan/templates"));

  let meta = fetch_meta(&options).unwrap();
  assert_eq!(meta.name, "default");
}
