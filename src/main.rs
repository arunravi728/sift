use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);

    println!("Searching for {} in {}", config.query, config.path);

    let contents =
        std::fs::read_to_string(config.path).expect("Should have been able to read the file");

    println!("File has contents -\n{contents}");
}

struct Config {
    query: String,
    path: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let path = args[2].clone();

        Config { query, path }
    }
}
