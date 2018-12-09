mod apply_filter;
mod array_walker;
mod cli;
mod core;
mod flatten_json_array;
mod get_selection;
mod group_walker;
mod parser;
mod range_selector;
mod types;
mod utils;

use clap::ArgMatches;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::prelude::Read;
use std::io::BufReader;
use std::path::Path;

/// Try to serialize the raw JSON content, output the selection or throw an
/// error.
fn output(json_content: &str, cli: &ArgMatches<'_>) {
    let inline = cli.is_present("inline");
    let selectors = cli.value_of("selectors");

    match serde_json::from_str(&json_content) {
        Ok(valid_json) => {
            // Walk through the JSON content with the provided selectors as
            // input.
            match core::walker(&valid_json, selectors) {
                Ok(selection) => println!(
                    "{}",
                    // Inline or pretty output.
                    (if inline {
                        serde_json::to_string
                    } else {
                        serde_json::to_string_pretty
                    })(&selection)
                    .unwrap()
                ),
                Err(error) => eprintln!("{}", error),
            }
        }
        Err(_) => eprintln!("Invalid JSON file or content"),
    }
}

fn main() {
    let cli = cli::get_matches();

    match cli.value_of("JSON") {
        // JSON content coming from the CLI.
        Some(json) => {
            let path = Path::new(json);
            let file = match File::open(&path) {
                Ok(file) => file,
                Err(_) => {
                    eprintln!("File {:?} not found", &path);
                    return;
                }
            };
            let mut buffer_reader = BufReader::new(file);
            let mut contents = String::new();

            match buffer_reader.read_to_string(&mut contents) {
                Ok(_) => output(&contents, &cli),
                Err(error) => panic!(
                    "Couldn't read {}: {}",
                    path.display(),
                    error.description()
                ),
            }
        }
        // JSON content coming from the stdin.
        None => {
            let mut json = String::new();
            match io::stdin().read_line(&mut json) {
                Ok(_) => output(&json, &cli),
                Err(error) => eprintln!("error: {}", error),
            }
        }
    }
}
