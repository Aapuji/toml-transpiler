use std::error::Error;

pub mod lexer;
pub mod parser;

pub enum Output {
    Success(Config),
    Help(String),
    InvalidArgCount
}

#[derive(Debug)]
pub struct Config {
    input_file: String,
    output_file: String
}

impl Config {
    pub fn build(args: &[String]) -> Output {
        for arg in args.iter() {
            if let "-h" | "--help" = arg.as_str() {
                return Output::Help(Config::generate_info());
            }
        }

        if args.len() < 3 {
            return Output::InvalidArgCount;
        }

        Output::Success(Config { 
            input_file: args[1].clone(), 
            output_file: args[2].clone() 
        })
    }

    fn generate_info() -> String {
        String::from("Usage: [options] <input-file> <output-file>\n\nOptions:\n  -h, --help\t\tPrint help\n")
    }

    pub fn run(self) -> Result<(), Box<dyn Error>> {
        todo!();
    }
}
