use crate::{config::Config, grep};
use std::{error::Error, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // read the file
    let contents = fs::read_to_string(config.file_path)?;

    // print the match pattern
    for line in grep::search(&config.pattern, &contents, config.case_insensitive) {
        println!("{line}");
    }

    Ok(())
}
