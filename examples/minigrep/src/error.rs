use std::error::Error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum AppError {
    MissingPattern,
    MissingFilename,
    ConfigLoad { source: io::Error },
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingPattern => f.write_str("No pattern provided"),
            Self::MissingFilename => f.write_str("No filename provided"),
            Self::ConfigLoad { source: _ } => f.write_str("Could not load config"),
        }
    }
}

impl From<io::Error> for AppError {
    fn from(source: io::Error) -> Self {
        Self::ConfigLoad { source }
    }
}

impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::ConfigLoad { source } => Some(source),
            _ => None,
        }
    }
}
