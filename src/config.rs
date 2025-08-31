use std::collections::HashSet;

pub struct Config {
    pub pattern: String,
    pub file_paths: Vec<String>,
    pub case_insensitive: bool,
    pub line_number: bool,
}

impl Config {
    /*
     * Build Config with arguments
     */
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        //Corner Case => Not Enough Arguments
        if args.len() < 3 {
            return Err(">> Not Enough Arguments!\n\
                 > example: cargo run -- test files/test.txt\n\
                 > example(case insensitive): cargo run -- TeSt files/test.txt -i");
        }

        let pattern: String = args[1].clone();
        let mut file_paths: HashSet<String> = HashSet::new(); //handle duplicate file paths with HashSet
        let mut case_insensitive: bool = false;
        let mut line_number: bool = false;

        for arg in &args[2..] {
            match arg.as_str() {
                "-i" => case_insensitive = true, // case insensitive -> flag '-i'
                "-n" => line_number = true, // show line number -> flag '-n'
                _ => {
                    file_paths.insert(arg.clone());
                }
            }
        }

        // Corner Case => No file paths
        if file_paths.is_empty() {
            return Err(">> No file(s) provided!");
        }

        Ok(Config {
            pattern,
            file_paths: file_paths.into_iter().collect(), //convert into vector
            case_insensitive,
            line_number,
        })
    }
}
