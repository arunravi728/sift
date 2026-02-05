use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let path = &args[2];

    println!("Searching for {query} in {path}");

    let contents = std::fs::read_to_string(path).expect("Should have been able to read the file");

    println!("File has contents -\n{contents}");
}
