use minigrep::{Config, run};
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        //exit without RUST_BACKTRACE message
        eprintln!("{err}");
        process::exit(1);
    });

    if let Err(err) = run(config) {
        //exit without RUST_BACKTRACE message
        eprintln!("{err}");
        process::exit(1);
    };
}
