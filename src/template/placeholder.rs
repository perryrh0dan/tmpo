use std::io::{Error};

use crate::template;

pub fn replace(data: &str, opts: &template::Options) -> Result<String, Error> {
    // replace placeholder with actual value
    let mut data = data.replace("{{name}}", &opts.name);
    if !opts.repository.is_none() {
        data = data.replace("{{repository}}", opts.repository.as_ref().unwrap());
    }
    if !opts.username.is_none() {
        data = data.replace("{{username}}", opts.username.as_ref().unwrap());
    }
    if !opts.email.is_none() {
        data = data.replace("{{email}}", opts.email.as_ref().unwrap());
    }
    Ok(data)
}
