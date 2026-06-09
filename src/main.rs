use std::env;

fn main() {
    let ar: Vec<String> = env::args().collect();
    dbg!(&ar);

    let query = &ar[1];
    let file_path = &ar[2];

    println!("Searching for {query}");
    println!("In file {file_path}");
    // println!("{:?}", ar);
}
