use std::process;

fn main() {
    if let Err(e) = minigrep::run() {
        println!("Application error: {}", e);

        process::exit(1);
    }
}
