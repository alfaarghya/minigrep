use minigrep::grep;
use std::{env, error::Error, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        //exit without RUST_BACKTRACE message
        eprintln!("{err}");
        process::exit(1);
    });

    if let Err(err) = run(config) {
        eprintln!("{err}");
        process::exit(1);
    };
}

struct Config {
    pattern: String,
    file_path: String,
    case_insensitive: bool,
}

impl Config {
    /*
     * Build Config with arguments
     */
    fn build(args: &[String]) -> Result<Config, &'static str> {
        // at-least 2 arguments needed
        if args.len() < 3 {
            return Err(
                ">> Not Enough Arguments!\n> example: cargo run -- test files/test.txt\n> example(case insensitive): cargo run -- TeSt files/test.txt -i",
            );
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

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    for line in grep::search(&config.pattern, &contents, config.case_insensitive) {
        println!("{line}");
    }

    Ok(())
}
