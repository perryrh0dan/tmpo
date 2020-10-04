use log;
use std::collections::HashMap;

extern crate handlebars;
use handlebars::Handlebars;

#[derive(Clone, serde::Serialize, Debug)]
pub struct Context {
  pub name: String,
  pub repository: String,
  pub username: String,
  pub email: String,
  pub values: HashMap<String, String>,
}

pub fn render(text: &str, content: &Context) -> String {
  // create the handlebars registry
  let handlebars = Handlebars::new();
  let context = match handlebars::Context::wraps(content) {
    Ok(context) => context,
    Err(error) => {
      log::error!("Error creating render context: Error: {}", error);
      return text.to_owned();
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_render_default() -> Result<(), Box<dyn std::error::Error>> {
    let text = "this is {{name}} an small test to test the basic {{repository}} features of the placeholder logic";
    let content: Context = Context {
      name: String::from("Tmpo"),
      repository: String::from("https://github.com/perryrh0dan/tmpo"),
      username: String::from("perryrh0dan"),
      email: String::from("thomaspoehlmann96@googlemail.com"),
      values: HashMap::new(),
    };

    let result = render(text, &content);

    assert_eq!(result, "this is Tmpo an small test to test the basic https://github.com/perryrh0dan/tmpo features of the placeholder logic");

    Ok(())
  }

  #[test]
  fn test_render_values() -> Result<(), Box<dyn std::error::Error>> {
    let text =
      "lets add one custom value: {{ values.full_name }} or a second one {{ values.last_name }}";
    let mut values = HashMap::new();
    values.insert(String::from("full_name"), String::from("Thomas Pöhlmann"));
    values.insert(String::from("last_name"), String::from("Pöhlmann"));
    let content: Context = Context {
      name: String::from("Tmpo"),
      repository: String::from("https://github.com/perryrh0dan/tmpo"),
      username: String::from("perryrh0dan"),
      email: String::from("thomaspoehlmann96@googlemail.com"),
      values: values,
    };

    let result = render(text, &content);

    assert_eq!(
      result,
      "lets add one custom value: Thomas Pöhlmann or a second one Pöhlmann"
    );

    Ok(())
  }

  #[test]
  fn test_render_unknown() -> Result<(), Box<dyn std::error::Error>> {
    let text = "lets add one custom value: {{ values.full_name }} or a second one {{ values.last_name }} and an unknown: {{ values.first_name }}";
    let mut values = HashMap::new();
    values.insert(String::from("full_name"), String::from("Thomas Pöhlmann"));
    values.insert(String::from("last_name"), String::from("Pöhlmann"));
    let content: Context = Context {
      name: String::from("Tmpo"),
      repository: String::from("https://github.com/perryrh0dan/tmpo"),
      username: String::from("perryrh0dan"),
      email: String::from("thomaspoehlmann96@googlemail.com"),
      values: values,
    };

    let result = render(text, &content);

    assert_eq!(
      result,
      "lets add one custom value: Thomas Pöhlmann or a second one Pöhlmann and an unknown: "
    );

    Ok(())
  }
}
