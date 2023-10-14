use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("Something went wrong while parsing/formatting time")]
    TimeError,
}
