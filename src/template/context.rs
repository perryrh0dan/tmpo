use std::collections::HashMap;

#[derive(Clone, serde::Serialize, Debug)]
pub struct Context {
  pub name: String,
  pub repository: String,
  pub username: String,
  pub email: String,
  pub values: HashMap<String, String>
}
