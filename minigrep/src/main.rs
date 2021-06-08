extern crate minigrep;

use std::env;
use std::process;

// If you use "minigrep::Config" when calling Config, you don't need to do this.
// Because Config is included in the import as "extern crate minigrep".
// However, by Rust convention, if the desired function is nested in two or more modules,
// the parent module is introduced into the scope.
use minigrep::Config;

fn main() {
    // Read the value of the command line argument.
    let args: Vec<String> = env::args().collect();

    // You can define your own error handling with unwrap_or_else.
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Apprication error: {}", e);
        process::exit(1);
    }
}

