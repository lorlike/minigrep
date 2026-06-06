use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);
    let contents = fs::read_to_string(config.filename).unwrap();

    for line in contents.lines() {
        if line.contains(&config.query) {
            println!("{line}");
        }
    }
}

struct Config {
    query: String,
    filename: String,
}
impl Config {
    fn new(args: &[String]) -> Self {
        let query = args[1].clone();
        let filename = args[2].clone();
        Config { query, filename }
    }
}
