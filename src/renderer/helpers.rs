extern crate convert_case;
use convert_case::{Case, Casing};

use handlebars::{
  to_json, Context, Decorator, Handlebars, Helper, JsonRender, Output, RenderContext, RenderError,
};

pub fn uppercase_helper(
  h: &Helper,
  _: &Handlebars,
  _: &Context,
  _: &mut RenderContext,
  out: &mut dyn Output,
) -> Result<(), RenderError> {
  // get parameter from helper or throw an error
  let param = h
      .param(0)
      .ok_or(RenderError::new("Param 0 is required for format helper."))?;

  let rendered = param.value().render().to_string().to_uppercase();
  out.write(rendered.as_ref())?;
  Ok(())
}

pub fn lowercase_helper(
  h: &Helper,
  _: &Handlebars,
  _: &Context,
  _: &mut RenderContext,
  out: &mut dyn Output,
) -> Result<(), RenderError> {
  // get parameter from helper or throw an error
  let param = h
      .param(0)
      .ok_or(RenderError::new("Param 0 is required for format helper."))?;

  let rendered = param.value().render().to_string().to_lowercase();
  out.write(rendered.as_ref())?;
  Ok(())
}

