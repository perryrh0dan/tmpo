use clap::Clap;
pub mod config;
mod init;

/// Search for a pattern in a file and display the lines that contain it
#[derive(Clap)]
#[clap(version = "0.1", author = "Thomas P.")]
struct Options {
    name: String,
    path: String,
}

fn main() {
    let opts: Options = Options::parse();
    let config = config::init().unwrap();

    let r = init::init_project(&config, &opts.path, &opts.name);
    match r {
        Ok(_v) => println!("Succesfull"),
        Err(_e) => println!("{}", _e)
    };
}

