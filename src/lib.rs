use parser::{DocumentParser, GlobalDocumentParser};
use std::error::Error;
use std::fs;

pub mod parser;
pub struct Config {
    src_filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Error reading the file ! - not enough arguments");
        }
        let src_filename = args[1].clone();
        Ok(Config { src_filename })
    }
}
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents: String = fs::read_to_string(config.src_filename)?;

    let mut parser: Option<Box<dyn DocumentParser>> =
        Some(Box::new(GlobalDocumentParser::new(vec![])));
    for line in contents.lines() {
        parser = Some(parser.take().unwrap().run_line(line));
    }

    parser.unwrap().to_html();
    Ok(())
}
