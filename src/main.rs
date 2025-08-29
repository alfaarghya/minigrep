use std::{env, fs};

fn main() {
    // passing arguments: cargo run -- test files/poem.txt
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    println!("Query => {}", config.query);
    println!("In File: {}", config.file_path);

    // read the file
    let content = fs::read_to_string(config.file_path)
        .expect("Should be able to read the file");

    println!("text: \n{content}");

}


struct Config {
    query: String,
    file_path: String
}

impl Config {
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!(">> Not Enough Arguments!\n> example: cargo run -- test files/test.txt");
        }

        let query: String = args[1].clone();
        let file_path: String = args[2].clone();

        Config {query, file_path}
    }
}
