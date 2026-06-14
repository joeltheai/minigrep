use minigrep::search;
use minigrep::search_case_insensitive;
use std::env;
use std::fs;

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
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        let (query, file_path, ignore_case) = match args.len() {
            1..=2 => return Err("Not enough arguments provided"),
            3 => (args[1].clone(), args[2].clone(), false),
            4 if args[3] == "-i" || args[3] == "--ignore-case" => {
                (args[1].clone(), args[2].clone(), true)
            }
            _ => {
                eprintln!("Usage: {} <query> <file> [-i|--ignore-case]", args[0]);
                return Err("unrecognized flag");
            }
        };

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    println!("Searching for : {}", config.query);

    println!("In file : {}", config.file_path);

    let contents = fs::read_to_string(config.file_path)?;

    // println!("\nContents:\n\n{contents}");

    println!("\n");
    if !config.ignore_case {
        for line in search(&config.query, &contents) {
            println!("{line}");
        }
    } else {
        for line in search_case_insensitive(&config.query, &contents) {
            println!("{line}");
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
