use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let config = match Config::new(env::args()) {
        Ok(config) => config,
        Err(err) => {
            eprintln!("Parse error: {}", err);
            process::exit(1);
        }
    };
    
    if let Err(err) = config.run() {
        eprintln!("Application error: {}", err);
        process::exit(1);
    }
}
