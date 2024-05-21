use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum Error {
    // time
    #[error("Something went wrong while parsing/formatting time")]
    TimeError,

    // link parsing
    #[error("The provided link is invalid")]
    InvalidLink,

    // yt-dlp
    #[error("The providad yt-dlp install and/or output path is invalid!")]
    InvalidYtDlpPath,
    #[error("Could not spawn the yt-dlp process with the specified parameters!")]
    YtDlpSpawnError,
    #[error("Could not read/write something into/from the process!\nReason: {0}")]
    YtDlpIOError(String),
    #[error("Process failed with exit code {0}!\nStderr: {1}")]
    YtDlpExitCode(i32, String),
    #[error("Could not process the output of the yt-dlp process!")]
    YtDlpOutputParseError,
}

#[derive(Error, Debug, Clone)]
pub enum RedisError {
    #[error("Failed to establish redis connection!")]
    RedisConnFailed,
    #[error("Failed to execute redis query!")]
    RedisQueryFailed,
}
