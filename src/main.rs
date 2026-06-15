use minigrep::search;
use minigrep::search_case_insensitive;
use std::env;
use std::fs;

use regex::Regex;
use walkdir::WalkDir;

// use std::io::ErrorKind;

fn main() {
    let ar: Vec<String> = env::args().collect();
    // dbg!(&ar);

    let config = Config::new(&ar).unwrap_or_else(|err| {
        println!("Error : {err}");
        std::process::exit(1);
    });

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        std::process::exit(1);
    }
}

struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
    invert: bool,
    count_only: bool,
    files_only: bool,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        let query;
        let file_path;
        let mut ignore_case = false;

        let mut invert = false;
        let mut count_only = false;
        let mut files_only = false;
        let mut pattern_index = 1;
        match args.len() {
            1..=2 => return Err("Usage: rgrep [OPTIONS] <pattern> <path>\nOptions: -v -c -l"),
            _ => {
                for i in 1..args.len() {
                    match args[i].as_str() {
                        "-v" => invert = true,
                        "-c" => count_only = true,
                        "-l" => files_only = true,
                        "-i" => ignore_case = true,
                        _ => {
                            pattern_index = i;
                            break;
                        }
                    }
                }

                if pattern_index + 1 >= args.len() {
                    return Err("Error: missing pattern or path");
                }

                query = args[pattern_index].clone();
                file_path = args[pattern_index + 1].clone();
            }
        };

        Ok(Config {
            query,
            file_path,
            ignore_case,
            invert,
            count_only,
            files_only,
        })
    }
}

fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    println!("Searching for : {}", config.query);

    println!("In file : {}", config.file_path);

    let regex = match Regex::new(&config.query) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Invalid pattern: {}", e);
            std::process::exit(1);
        }
    };

    for entry in WalkDir::new(&config.file_path).into_iter() {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };

        if !entry.file_type().is_file() {
            continue;
        }

        let file_pathy = entry.path();
    } // println!("\nContents:\n\n{contents}");

    let contents = fs::read_to_string(config.file_path)?;
    println!("\n");
    if !config.ignore_case {
        for line in search(&contents, &regex, config.invert) {
            println!("{:?}", line);
        }
    } else {
        for line in search_case_insensitive(&contents, &regex, config.invert) {
            println!("{:?}", line);
        }
    }

    Ok(())

    // match fs::read_to_string(config.file_path) {
    //     Ok(contents) => {
    //         println!("Contents:\n{contents}");
    //     }
    //     Err(other_err) => {
    //         eprintln!("{}", other_err);
    //     }
    // }
}
