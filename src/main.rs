use minigrep::search;
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

#[derive(Debug)]
struct Config {
    query: String,
    file_path: String,
    invert: bool,
    count_only: bool,
    files_only: bool,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        let query;
        let file_path;

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
            invert,
            count_only,
            files_only,
        })
    }
}

fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let regex = match Regex::new(&config.query) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Invalid pattern: {}", e);
            std::process::exit(1);
        }
    };

    let search_multiple = fs::metadata(&config.file_path)?.is_dir();

    for entry in WalkDir::new(&config.file_path).into_iter() {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };

        if !entry.file_type().is_file() {
            continue;
        }

        let path = entry.path();
        let contents = match fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => continue,
        };
        let matches = search(&contents, &regex, config.invert);

        if config.files_only {
            if !matches.is_empty() {
                println!("{}", path.display());
            }
        } else if config.count_only {
            if search_multiple {
                println!("{}:{}", path.display(), matches.len());
            } else {
                println!("{}", matches.len());
            }
        } else {
            for (line_num, line) in matches {
                println!("{}:{}", line_num, line);
            }
        }
    }

    Ok(())
}
