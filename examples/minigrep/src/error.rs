use std::io;
use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum AppError {
    MissingQuery,
    MissingFilename,
    ConfigLoad { source: io::Error },
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingQuery=> f.write_str("Didn't get a query string"),
            Self::MissingFilename => f.write_str("Didn't get a file name"),
            Self::ConfigLoad { source } => write!(f, "Could not load config: {}", source),
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
