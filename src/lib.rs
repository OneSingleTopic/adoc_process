use handlebars::{no_escape, Handlebars};
use parser::{DocumentParser, GlobalDocumentParser};
use std::collections::BTreeMap;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{ErrorKind, Write};

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
    let mut handlebars = Handlebars::new();
    handlebars.register_escape_fn(no_escape);
    handlebars
        .register_template_file("document", "templates/document.html")
        .unwrap();

    let contents: String = fs::read_to_string(config.src_filename)?;

    let mut parser: Option<Box<dyn DocumentParser>> =
        Some(Box::new(GlobalDocumentParser::new(vec![])));
    for line in contents.lines() {
        parser = Some(parser.take().unwrap().run_line(line));
    }

    let mut document = BTreeMap::new();
    document.insert("title".to_string(), "Coucou".to_string());
    document.insert("body".to_string(), parser.unwrap().to_html());
    let document = handlebars.render("document", &document)?;

    let mut file = open_a_file("result.html");
    file.write(document.as_bytes())
        .expect("Problem writing the file !");

    Ok(())
}
fn open_a_file(filepath: &str) -> File {
    let f = File::open(filepath);

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(filepath) {
                Ok(file) => file,
                Err(err) => panic!("{:?}", err),
            },
            other_error => {
                panic!("Error opening the file : {:?}", other_error)
            }
        },
    };
    f
}
