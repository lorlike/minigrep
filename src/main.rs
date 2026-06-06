use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    let contents = fs::read_to_string(filename).unwrap();

    for line in contents.lines() {
        if line.contains(query) {
            println!("{line}");
        }
    }
}
