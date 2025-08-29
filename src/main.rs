use std::{env, error::Error, fs, process};

fn main() {
    // passing arguments: cargo run -- test files/poem.txt
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        //exit without RUST_BACKTRACE message
        println!("{err}");
        process::exit(1);
    });

    if let Err(err) = run(config) {
        println!("{err}");
        process::exit(1);
    };

}


struct Config {
    query: String,
    file_path: String
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err(">> Not Enough Arguments!\n> example: cargo run -- test files/test.txt");
        }

        let query: String = args[1].clone();
        let file_path: String = args[2].clone();

        Ok(Config {query, file_path})
    }
}


fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("Query => {}", config.query);
    println!("In File: {}", config.file_path);

    // read the file
    let content = fs::read_to_string(config.file_path)?;

    println!("text: \n{content}");

    Ok(())
}
