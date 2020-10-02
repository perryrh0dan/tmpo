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
  let mut index = 0;
  for text in vec {
    if index == vec.len() - 1{
      result += text;
    } else {
      result = result + text + ", ";
    }
    index += 1;
  }

  return result;
}

#[test]
fn test_capitalize() -> Result<(), Box<dyn std::error::Error>> {
  let test: &str = "extraterrestrial";

  let result = capitalize(test);

  assert_eq!(result, "Extraterrestrial");

  Ok(())
}

#[test]
fn test_capitalize_two() -> Result<(), Box<dyn std::error::Error>> {
  let test: &str = "Extraterrestrial";

  let result = capitalize(test);

  assert_eq!(result, "Extraterrestrial");

  Ok(())
}

#[test]
fn test_lowercase() -> Result<(), Box<dyn std::error::Error>> {
  let test: &str = "ExtrateRRestrIAl";

  let result = lowercase(test);

  assert_eq!(result, "extraterrestrial");

  Ok(())
}

#[test]
fn test_vec_to_string() -> Result<(), Box<dyn std::error::Error>> {
  let test: Vec<String> = vec!{
    String::from("proper"),
    String::from("unlike"),
    String::from("garlic"),
  };

  let result = vec_to_string(&test);

  assert_eq!(result, "proper, unlike, garlic");

  Ok(())
}
