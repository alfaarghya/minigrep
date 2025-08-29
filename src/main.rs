use std::{env, fs};

fn main() {
    // passing arguments: cargo run -- test files/poem.txt
    let args: Vec<String> = env::args().collect();

    // query and file path from arguments
    let query: &String = &args[1];
    let file_path: &String = &args[2];

    println!("Query => {query}");
    println!("In File: {file_path}");

    // read the file
    let content = fs::read_to_string(file_path)
        .expect("Should be able to read the file");

    println!("text: \n{content}");

}
