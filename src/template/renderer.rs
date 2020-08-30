use log;

use handlebars::Handlebars;

use crate::template::context;

pub fn render(text: &str, opts: &context::Context) -> String {
    // create the handlebars registry
    let handlebars = Handlebars::new();

    let result = match handlebars.render_template(text, opts) {
        Ok(result) => result,
        Err(_error) => { 
            log::error!("Error rendering template");
            text.to_owned()
        }
    };

    return result;
}