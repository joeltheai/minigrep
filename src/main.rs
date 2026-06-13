use std::env;
use std::fs;

// use std::io::ErrorKind;

fn main() {
    let ar: Vec<String> = env::args().collect();
    dbg!(&ar);

    let (query, file_path) = parse_config(&ar);
    println!("Searching for {query}");
    println!("In file {file_path}");

    // println!("{:?}", ar);

    println!("In file {file_path}");

    // let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    // println!("With text:\n{contents}");

    // 3. Replaced .expect() with a match statement to handle errors gracefully
    match fs::read_to_string(file_path) {
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

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let file_path = &args[2];

    (query, file_path)
}
