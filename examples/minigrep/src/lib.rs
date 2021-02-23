use std::fs;
use std::env;
use std::io;

use thiserror::Error;

pub struct Config {
    pattern: String,
    filename: String,
    case_insensitivity: bool,
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error("No pattern provided")]
    MissingPattern,
    #[error("No filename provided")]
    MissingFilename,
    #[error("File read error")]
    Io {
        #[from]
        source: io::Error
    }
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Self, AppError> {
        args.next(); 

        let pattern = match args.next() {
            Some(arg) => arg,
            None => return Err(AppError::MissingPattern),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err(AppError::MissingFilename),
        };

        let case_insensitivity = env::var("CASE_INSENSITIVE").is_ok();

        Ok(Config {
            pattern,
            filename,
            case_insensitivity,
        })
    }
    
    pub fn run(&self) -> Result<(), AppError> {
        let contents = fs::read_to_string(&self.filename)?;

        let found = if self.case_insensitivity {
            search_insensitive(&self.pattern, &contents) 
        } else {
            search(&self.pattern, &contents) 
        };
        
        for line in found {
            println!("{}", line);
        }

        Ok(())
    }
}
 
fn search<'a>(pattern: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()                
        .filter(|line| line.contains(pattern))  // filter out any lines that don't contain the pattern
        .collect()
}

fn search_insensitive<'a>(pattern: &str, contents: &'a str) -> Vec<&'a str> {
    let pattern = pattern.to_lowercase();
    
    contents.lines()
        .filter(|line| line.to_lowercase().contains(&pattern)) 
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn no_results() {
        let pattern = "x";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
    
        let found = search(pattern, contents);
        assert_eq!(found.len(), 0);
    }

    #[test]
    fn with_results() {
        let pattern = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        let found = search(pattern, contents);
        assert_eq!(vec!["safe, fast, productive."], found);
    }

    #[test]
    fn case_sensitivity() {
        let pattern = "Rust";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        let found = search(pattern, contents);
        assert_eq!(vec!["Rust:"], found); 
    }

    #[test]
    fn case_insensitivity() {
        let pattern = "pick";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        let found = search_insensitive(pattern, contents);
        assert_eq!(vec!["Pick three."], found);
    }
}
