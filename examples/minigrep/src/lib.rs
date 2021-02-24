mod error;

use error::AppError;
use std::env;
use std::fs;

pub struct Config {
    pattern: String,
    filename: String,
    case_insensitivity: bool,
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

pub fn search<'a>(pattern: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(pattern))
        .collect()
}

pub fn search_insensitive<'a>(pattern: &str, contents: &'a str) -> Vec<&'a str> {
    let pattern = pattern.to_lowercase();

    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&pattern))
        .collect()
}
