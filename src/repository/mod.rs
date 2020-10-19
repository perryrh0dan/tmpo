use std::collections::HashSet;
use std::fs;
use std::fs::File;
use std::io::{Error, Write};
use std::path::{Path, PathBuf};

use crate::config::{Config, RepositoryOptions};
use crate::context::Context;
use crate::error::RunError;
use crate::git;
use crate::meta;
use crate::template;
use crate::template::{renderer, Template};
use crate::utils;

pub mod custom_repository;
pub mod default_repository;

pub trait Repository {
  fn get_config(&self) -> RepositoryOptions;
  fn copy_template(&self, ctx: &Context, opts: &CopyOptions) -> Result<(), RunError>;
  fn get_template_values(&self, template_name: &str) -> Result<HashSet<String>, RunError>;
  fn get_template_names(&self) -> Vec<String>;
  fn get_template_by_name(&self, name: &str) -> Result<&template::Template, RunError>;
}

#[derive(Debug)]
pub struct CopyOptions {
  pub template_name: String,
  pub target: PathBuf,
  pub render_context: renderer::Context,
}
