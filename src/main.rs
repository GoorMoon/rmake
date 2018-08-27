#[macro_use]
extern crate clap;
extern crate rmake_lib;

use clap::{App, Arg};
use rmake_lib::validators::path_is_exists;

pub struct MakeOptions{
    makefile: String,
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
