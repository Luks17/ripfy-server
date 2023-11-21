use super::error::Error;
use lazy_regex::regex_captures;

pub fn parse_yt_link(link: &str) -> Result<String, Error> {
    let (_, id) =
        regex_captures!(r#"(?:watch\?v=|youtu\.be\/)([\w-]+)"#, link).ok_or(Error::InvalidLink)?;

    Ok(id.to_string())
}
