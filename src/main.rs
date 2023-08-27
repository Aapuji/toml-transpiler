use std::env;
use std::process;

use toml_transpiler::{Config, Output};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = match Config::build(&args) {
        Output::Success(cfg) => cfg,
        Output::Help(msg) => {
            println!("{msg}");
            process::exit(0);
        },
        Output::InvalidArgCount => {
            eprintln!("Invalid number of arguments.");
            process::exit(1);
        }
    };

    println!("{:#?}", config);

    process::exit(0);
}
