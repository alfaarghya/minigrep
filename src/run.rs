use crate::{config::Config, grep};
use std::{error::Error, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    for file_path in config.file_paths {
        // read the file
        let contents: String = fs::read_to_string(&file_path)?;

        // all match lines
        let results: Vec<(usize, &str)> =
            grep::search(&config.pattern, &contents, config.case_insensitive);

        // results empty -> no matches on display
        if !results.is_empty() {
            // result available -> display matches on screen
            println!("--- Matches in {file_path} ---");
            for (line_num, line) in results {
                if config.line_number {
                    println!("{}:{}", line_num, line);
                } else {
                    println!("{line}");
                }
            }
        }
    }

    Ok(())
}
