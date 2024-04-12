use super::error::Error;
use crate::config;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    fmt::Display,
    path::{Path, PathBuf},
    process::{ExitStatus, Stdio},
    time::Duration,
};
use tokio::{
    io::AsyncReadExt,
    process::{Child, Command},
    time::timeout,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct YtDlpResult {
    pub channel: String,
    pub fulltitle: String,
}

impl YtDlpResult {
    fn from_stdout(stdout: Vec<u8>) -> Result<Self, Error> {
        let value: Value =
            serde_json::from_reader(stdout.as_slice()).map_err(|_| Error::YtDlpOutputParseError)?;

        let result: Self =
            serde_json::from_value(value).map_err(|_| Error::YtDlpOutputParseError)?;

        Ok(result)
    }
}

impl Display for YtDlpResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Channel: {}\nTitle: {}", self.channel, self.fulltitle)
    }
}

/// Creates a new yt-dlp process template
/// Requires a yt-dlp binary in the install_path and ffmpeg
#[derive(Debug)]
pub struct YtDlp {
    install_path: PathBuf,
    output_path: PathBuf,
    timeout: Duration,
}

impl Default for YtDlp {
    fn default() -> Self {
        Self {
            install_path: Path::new(&config().yt_dlp_binary_path).to_path_buf(),
            output_path: Path::new(&config().yt_dlp_output_path).to_path_buf(),
            timeout: Duration::from_millis(config().yt_dlp_timeout_milisecs),
        }
    }
}

impl YtDlp {
    /// Receives a yt video id as parameter
    /// Downloads video from url, extracts audio as opus and outputs it to the output_path
    /// Returns some info about the video as a YtDlpOutput
    pub async fn run(&self, id: &str) -> Result<YtDlpResult, Error> {
        let url = get_url(id);

        // extract audio - convert to opus - output to output_path - name is the video id
        let args = vec![
            "--print",
            "before_dl:%(.{channel,fulltitle})#j",
            "-x",
            "--audio-format",
            "opus",
            "-P",
            self.output_path.to_str().ok_or(Error::InvalidYtDlpPath)?,
            "-o",
            "%(id)s",
            &url,
        ];

        let mut child = self.spawn_child(args).await?;
        let exit_code = self.execute_until_exit(&mut child).await?;

        // reads stdout and stderr
        let stdout = read_std_buffer(child.stdout).await?;
        let stderr = read_std_buffer(child.stderr).await?;

        // if the process did not exit with exit code 0, we return early
        if !exit_code.success() {
            let stderr = String::from_utf8(stderr).unwrap_or_default();
            return Err(Error::YtDlpExitCode(exit_code.code().unwrap_or(1), stderr));
        }

        let yt_dlp_result = YtDlpResult::from_stdout(stdout)?;

        Ok(yt_dlp_result)
    }

    /// spawns child process
    async fn spawn_child(&self, args: Vec<&str>) -> Result<Child, Error> {
        let child = Command::new(self.install_path.to_str().ok_or(Error::InvalidYtDlpPath)?)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .args(args)
            .spawn()
            .map_err(|_| Error::YtDlpSpawnError)?;

        Ok(child)
    }

    /// Tries to run the process until exit
    /// Process WILL timeout if it takes longer than self.timeout duration
    async fn execute_until_exit(&self, child: &mut Child) -> Result<ExitStatus, Error> {
        let exit_code = match timeout(self.timeout, child.wait()).await {
            Ok(result) => result.map_err(|e| Error::YtDlpIOError(e.to_string()))?,
            Err(_) => {
                child
                    .kill()
                    .await
                    .map_err(|e| Error::YtDlpIOError(e.to_string()))?;
                return Err(Error::YtDlpExitCode(
                    1,
                    "Yt-dlp process timeout".to_string(),
                ));
            }
        };

        Ok(exit_code)
    }
}

/// Reads stdout and stderr from child processes
async fn read_std_buffer<T: AsyncReadExt + Unpin>(reader: Option<T>) -> Result<Vec<u8>, Error> {
    let mut output = vec![];

    if let Some(mut reader) = reader {
        reader
            .read_to_end(&mut output)
            .await
            .map_err(|e| Error::YtDlpIOError(e.to_string()))?;
    }

    Ok(output)
}

fn get_url(id: &str) -> String {
    "https://www.youtube.com/watch?v=".to_string() + id
}
