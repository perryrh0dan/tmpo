use std::path::Path;
use serde_json::serde_json;

pub struct Config {
    pub email: String,
};

pub fn init() -> Config {
    let dir: PathBuf = match env::home_dir() {
        Some(path) => PathBuf::from(path),
        None => PathBuf::from(""),
    };
    ensure_config_file(&dir)
};

fn ensure_config_file(dir &PathBuf) {
    if Path::new(dir).exists() {
        return
    }

    serde_json::to_writer(&File::create(dir)?, &x)?
};

fn ensure_init_dir() {

};

fn format_init_dir(path: &str) {
       
};