use thiserror::Error;

#[derive(Debug, Error)]
pub enum CliError {
    #[error("Bad parameter: {0}")]
    BadParameter(String),
    #[error("Command not recognized: {0}")]
    CommandNotRecognized(String),
}
