#[macro_use]
extern crate clap;

use clap::{App, Arg};
use std::path::Path;

fn path_is_exists(path: String) -> Result<(), String> {
    if Path::new(&path).is_file() {
        Ok(())
    } else {
        Err(format!("The file <{}> doesn't exists", path))
    }
}

fn main() {
    let yaml_config = load_yaml!("config/cli.yml");
    let matches = App::from_yaml(yaml_config)
        .version(crate_version!())
        .arg(
            Arg::from_usage("-f, --file <FILE> 'Read the file named FILE as a makefile.")
                .required(false)
                .validator(path_is_exists),
        )
        .get_matches();

    if let Some(file) = matches.value_of("file") {
        println!("The file is {}", file);
    }
}
