use adoc_process;
use adoc_process::Config;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("Asciidoc Processor");

    let config = Config::new(&args).unwrap_or_else(|error| {
        eprintln!("Problem parsing the arguments ! : {} ", error);
        process::exit(1);
    });

    if let Err(error) = adoc_process::run(config) {
        eprintln!("Problem reading the document ! : {} ", error);
        process::exit(1);
    };
}
