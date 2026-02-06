use std::env;
use std::error::Error;

use sift::search;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Argument parsing returned an error: {err}");
        std::process::exit(1);
    });

    println!("Searching for {} in {}", config.query, config.path);

    if let Err(err) = config.run() {
        println!("Running config returned an error: {err}");
        std::process::exit(1);
    }
}

struct Config {
    query: String,
    path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, String> {
        if args.len() < 3 {
            return Err("Must provide atleast 2 arguments for query and file name.".to_string());
        }
        let query = args[1].clone();
        let path = args[2].clone();

        Ok(Config { query, path })
    }

    fn run(&self) -> Result<(), Box<dyn Error>> {
        let contents = std::fs::read_to_string(self.path.clone())?;

        for line in search(&self.query, &contents) {
            println!("{line}");
        }

        Ok(())
    }
}
