use std::{error::Error, fs};

use clap::Parser;

/// minigrep by lorlike
#[derive(Debug, Parser)]
#[command(name = "minigrep", version)]
pub struct Args {
    /// the string for query
    query: String,
    /// the filename to query
    filename: String,
    /// match insensitively
    #[arg(long, short)]
    case_insensitive: bool,
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let contents = fs::read_to_string(args.filename)?;

    let results = if args.case_insensitive {
        search_case_insensitive(&args.query, &contents)
    } else {
        search(&args.query, &contents)
    };
    for line in results {
        println!("{line}");
    }
    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
