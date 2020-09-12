use log;

use crate::template::context;

extern crate handlebars;
extern crate serde_json;

use handlebars::{Handlebars, Context};

pub fn render(text: &str, opts: &context::Context) -> String {
    // create the handlebars registry
    let handlebars = Handlebars::new();
    let context = match Context::wraps(opts) {
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
