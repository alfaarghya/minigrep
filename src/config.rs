pub struct Config {
    pub pattern: String,
    pub file_path: String,
    pub case_insensitive: bool,
}

impl Config {
    /*
     * Build Config with arguments
     */
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        // at-least 2 arguments needed
        if args.len() < 3 {
            return Err(">> Not Enough Arguments!\n\
                 > example: cargo run -- test files/test.txt\n\
                 > example(case insensitive): cargo run -- TeSt files/test.txt -i");
        }

        let pattern: String = args[1].clone();
        let file_path: String = args[2].clone();
        let mut case_insensitive: bool = false;

        for arg in &args[1..] {
            if arg == "-i" {
                case_insensitive = true;
            }
        }

        Ok(Config {
            pattern,
            file_path,
            case_insensitive,
        })
    }
}
