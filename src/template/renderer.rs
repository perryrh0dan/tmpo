use log;
use std::collections::HashMap;

extern crate handlebars;
use handlebars::{Handlebars};

#[derive(Clone, serde::Serialize, Debug)]
pub struct Context {
  pub name: String,
  pub repository: String,
  pub username: String,
  pub email: String,
  pub values: HashMap<String, String>
}


pub fn render(text: &str, content: &Context) -> String {
    // create the handlebars registry
    let handlebars = Handlebars::new();
    let context = match handlebars::Context::wraps(content) {
      Ok(context) => context,
      Err(error) => {
        log::error!("Error creating render context: Error: {}", error);
        return text.to_owned()
      }
    };

    let result = match handlebars.render_template_with_context(text, &context) {
        Ok(result) => result,
        Err(error) => {
            log::error!("Error rendering template: Error: {}", error);
            text.to_owned()
        }
    };

    return result;
}
