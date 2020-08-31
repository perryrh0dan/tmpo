use colored::Colorize;

pub fn app_updated() {
    let text = format!("Successful updated app").green();
    println!("{}", text);
}

pub fn workspace_created(name: &str) {
    let text = format!("Created workspace: {}", name).green();
    println!("{}", text);
}

pub fn template_created(path: &str) {
    let text = format!("Created template: {}", path).green();
    println!("{}", text);
}
