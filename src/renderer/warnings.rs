use colored::Colorize;

pub fn no_subcommand() {
  let text = format!("No subcommand was used").yellow();
  println!("{}", text);
}