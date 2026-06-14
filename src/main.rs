use minigrep::search;
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
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments provided");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    println!("Searching for {}", config.query);

    println!("In file {}", config.file_path);

    let contents = fs::read_to_string(config.file_path)?;

    for line in search(&config.query, &contents) {
        println!("{line}");
    }

    println!("Contents:\n{contents}");
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
