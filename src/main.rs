use colored::Colorize;
use std::{env, fs};
use whatlang::detect;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!(
            "{}",
            format!("Error: Please pass a argument: text or text file path").red()
        );
        return;
    }

    let query = &args[1];

    if query.as_bytes().ends_with(b".txt") {
        let contents = match fs::read_to_string(query) {
            Ok(contents) => contents,
            Err(_) => {
                println!(
                    "{}",
                    format!("Error: Unable to read file. Make sure the text file exists.").red()
                );
                return;
            }
        };
        guess_language(&contents)
    } else {
        guess_language(query)
    }
}

fn guess_language(query: &String) {
    println!("\n{}", format!("----Contents----").bold().blue());
    println!("{}", query);

    let info = detect(query).unwrap();
    println!("\n{}", format!("---Language----").bold().blue());
    println!("Lang: {}", info.lang());
    println!("Script: {}", info.script());
    println!("confidence: {}", info.confidence());
    println!("Is Reliable: {}", info.is_reliable());
}
