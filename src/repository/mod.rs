use linked_hash_set::LinkedHashSet;
use std::path::{PathBuf};

use crate::config::{RepositoryOptions};
use crate::context::Context;
use crate::error::RunError;
use crate::meta::Value;
use crate::template;
use crate::renderer;

pub mod remote_repository;
pub mod default_repository;
pub mod external_repository;

pub trait Repository {
  fn get_config(&self) -> RepositoryOptions;
  fn copy_template(&self, ctx: &Context, opts: &CopyOptions) -> Result<(), RunError>;
  fn get_template_info(&self, template_name: &str) -> Result<Option<String>, RunError>;
  fn get_template_values(&self, template_name: &str) -> Result<LinkedHashSet<Value>, RunError>;
  fn get_template_names(&self) -> Vec<String>;
  fn get_template_by_name(&self, name: &str) -> Result<&template::Template, RunError>;
}

#[derive(Debug)]
pub struct CopyOptions {
  pub template_name: String,
  pub target: PathBuf,
  pub render_context: renderer::Context,
}
