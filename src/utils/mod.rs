pub fn capitalize(text: &str) -> String {
  let mut v: Vec<char> = text.chars().collect();
  v[0] = v[0].to_uppercase().nth(0).unwrap();
  return v.into_iter().collect();
}

pub fn lowercase(text: &str) -> String {
  let mut v: Vec<char> = text.chars().collect();
  for i in v.iter_mut() {
    *i = i.to_lowercase().nth(0).unwrap();
  }
  return v.into_iter().collect();
}

pub fn vec_to_string(vec: &Vec<String>) -> String {
  let mut result = String::new();
  for text in vec {
    result = result + text;
  }

  return result;
}
