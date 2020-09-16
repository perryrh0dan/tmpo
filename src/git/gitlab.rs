use crate::error::RunError;
use crate::git;
use crate::meta::Meta;

extern crate reqwest;
extern crate regex;
use regex::Regex;
extern crate url;
use url::form_urlencoded::{byte_serialize};
extern crate serde_json;
use serde::{Deserialize};
extern crate base64;

#[derive(Deserialize, Debug)]
struct FileResponse {
  file_name: String,
  file_path: String,
  size: usize,
  encoding: String,
  content_sha256: String,
  r#ref: String,
  blob_id: String,
  commit_id: String,
  last_commit_id: String,
  content: String
}

pub fn fetch_meta(options: &git::Options) -> Result<Meta, RunError> {
  let file_response = match fetch(options) {
    Ok(resp) => resp,
    Err(error) => return Err(error),
  };

  let decoded_content = match base64::decode(file_response.content) {
    Ok(data) => data,
    Err(error) => return Err(RunError::Meta(format!("{}", error))),
  };

  let meta: Meta = match serde_json::from_slice(&decoded_content) {
    Ok(meta) => meta,
    Err(error) => return Err(RunError::Meta(format!("{}", error))),
  };

  Ok(meta)
}

fn fetch(options: &git::Options) -> Result<FileResponse, RunError> {
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

  if provider != git::Provider::GITLAB {
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
      "PRIVATE-TOKEN",
      options.token.clone().unwrap().parse().unwrap(),
    );
  }

  let client = reqwest::blocking::Client::new();
  let response = match client.get(&meta_url).headers(headers).send() {
    Ok(resp) => resp,
    Err(error) => return Err(RunError::Meta(format!("{}", error))),
  };

  let file_response: FileResponse = match response.json() {
    Ok(data) => data,
    Err(error) => return Err(RunError::Git(format!("Cant deserialize data: {}", error))),
  };

  Ok(file_response)
}

pub fn build_meta_url(repository_url: &str) -> Result<String, RunError> {
  // Target: https://gitlab.com/api/v4/projects/JohnMcClan3%2Ftemplates/repository/files/meta.json?ref=master
  // Extract the domain
  let re = Regex::new("(http://)?(https://)?[^/]+").unwrap();
  let domain: String = match re.find(repository_url) {
    Some(value) => String::from(value.as_str()),
    None => return Err(RunError::Git(String::from("Cant extract domain"))),
  };

  // Extract the repository path
  let re = Regex::new("http[s]?://[^/]+/(.+)").unwrap();
  let captures = match re.captures(repository_url) {
    Some(value) => value,
    None => return Err(RunError::Git(String::from("Cant extract path")))
  };

  let path: String = match captures.get(1) {
    Some(value) => String::from(value.as_str()),
    None => return Err(RunError::Git(String::from("Cant extract path")))
  };

  // URLEncode the path
  let urlencoded_path: String = byte_serialize(path.as_bytes()).collect();

  // Build the meta_url
  let meta_url = format!("{}/api/v4/projects/{}/repository/files/meta.json?ref=master", domain, urlencoded_path);

  Ok(meta_url)
}

#[test]
fn build_meta_url_default() {
  let repository_url = "https://gitlab.com/JohnMcClan3/templates";

  let url = build_meta_url(repository_url);
  assert_eq!(url.unwrap(), "https://gitlab.com/api/v4/projects/JohnMcClan3%2Ftemplates/repository/files/meta.json?ref=master");
}

#[test]
fn build_meta_url_http() {
  let repository_url = "http://gitlab.com/JohnMcClan3/templates";

  let url = build_meta_url(repository_url);
  assert_eq!(url.unwrap(), "http://gitlab.com/api/v4/projects/JohnMcClan3%2Ftemplates/repository/files/meta.json?ref=master");
}

#[test]
fn build_meta_url_ce() {
  let repository_url = "https://gitlab1.camelot-idpro.de/developmentgovernance/templates";

  let url = build_meta_url(repository_url);
  assert_eq!(url.unwrap(), "https://gitlab1.camelot-idpro.de/api/v4/projects/developmentgovernance%2Ftemplates/repository/files/meta.json?ref=master");
}

#[test]
fn fetch_meta_success() {
  let mut options = git::Options::new();
  options.enabled = true;
  options.provider = Some(git::Provider::GITLAB);
  options.auth = Some(git::AuthType::TOKEN);
  options.token = Some(String::from("r-6fZ-CXscYu97u4m-mD"));
  options.url = Some(String::from("https://gitlab.com/JohnMcClan3/templates"));

  let meta = fetch_meta(&options).unwrap();
  assert_eq!(meta.name, "test");
}

#[test]
fn fetch_meta_failure() {
  let mut options = git::Options::new();
  options.enabled = true;
  options.provider = Some(git::Provider::GITLAB);
  options.auth = Some(git::AuthType::TOKEN);
  options.url = Some(String::from("https://gitlab.com/JohnMcClan3/templates"));

  let meta = fetch_meta(&options);
  match meta {
    Ok(_) => assert!(false),
    Err(_) => assert!(true),
  }
}
