/// Get the global git email
pub fn get_email() -> Result<String, git2::Error> {
  let config = get_config()?;
  let email = config.get_string("user.email")?;

  let mut buf = String::with_capacity(email.len());

  for c in email.chars() {
    buf.push(c);
  }

  Ok(buf)
}

/// Get get global git username
pub fn get_username() -> Result<String, git2::Error> {
  let config = get_config()?;
  let username = config.get_string("user.name")?;

  let mut buf = String::with_capacity(username.len());

  for c in username.chars() {
    buf.push(c);
  }

  Ok(buf)
}

/// load global git config
fn get_config() -> Result<git2::Config, git2::Error> {
  let path = git2::Config::find_global()?;
  let config = git2::Config::open(&path)?;

  // for entry in &config.entries(None).unwrap() {
  //     let entry = entry.unwrap();
  //     println!("{} => {}", entry.name().unwrap(), entry.value().unwrap());
  // }
  Ok(config)
}
