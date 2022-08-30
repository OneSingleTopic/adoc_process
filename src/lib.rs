use std::error::Error;
use std::fs;

pub enum AdocObject {
    Paragraph(String),
    Title(String, usize),
}

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

    for line in contents.lines() {
        let mut vector = line.split(" ");
        if let Some(first_element) = vector.next() {
            if first_element.contains("=") {
                let mut title_level = 0;
                let mut title_indicator = first_element.chars();
                while let Some('=') = title_indicator.next() {
                    title_level += 1;
                }

                println!(
                    "Title level {} : {}",
                    title_level,
                    vector.collect::<Vec<&str>>().join(" ")
                );
            } else {
                println!(
                    "{} {}",
                    first_element,
                    vector.collect::<Vec<&str>>().join(" ")
                );
            }
        } else {
            println!("{}", line);
        }
    }

    Ok(())
}
