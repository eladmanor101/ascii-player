use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("Application error: {0}")]
    Application(String),

    #[error("Pipeline error: {0}")]
    Pipeline(String),

    #[error("Player error: {0}")]
    Player(String)
}

impl From<io::Error> for MyError {
    fn from(error: io::Error) -> Self {
        MyError::Application(format!("{error}"))
    }
}

impl From<serde_json::Error> for MyError {
    fn from(error: serde_json::Error) -> Self {
        MyError::Application(format!("{error}"))
    }
}