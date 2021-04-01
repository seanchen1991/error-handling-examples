use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Didn't get a query string")]
    MissingQuery,
    #[error("Didn't get a file name")]
    MissingFilename,
    #[error("Could not load config: {source:?}")]
    ConfigLoad {
        #[from] 
        source: io::Error,
    }
}
