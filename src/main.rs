use std::env;
use std::fs;

// use std::io::ErrorKind;

fn main() {
    let ar: Vec<String> = env::args().collect();
    dbg!(&ar);

    let config = Config::new(&ar).unwrap_or_else(|err| {
        println!("Error : {err}");
        std::process::exit(1);
    });

    run(config);
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

fn run(config: Config) {
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    // println!("{:?}", ar);

    println!("In file {}", config.file_path);

    // let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    // println!("With text:\n{contents}");

    match fs::read_to_string(config.file_path) {
        Ok(contents) => {
            println!("With text:\n{contents}");
        }
        Err(other_err) => {
            eprintln!("{}", other_err);
        } // Err(err) if err.kind() == ErrorKind::NotFound => {
          //     eprintln!("Error: The file '{}' does not exist.", file_path);
          //     // Program exits cleanly here instead of crashing with a panic
          // }
    }
}
