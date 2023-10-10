use time::{format_description::well_known::Rfc3339, Duration, OffsetDateTime};

use super::error::Error;

pub fn now_utc() -> OffsetDateTime {
    OffsetDateTime::now_utc()
}

pub fn utc_time_to_str(time: OffsetDateTime) -> Result<String, Error> {
    let formatted = time.format(&Rfc3339).map_err(|_| Error::TimeError)?;

    Ok(formatted)
}

pub fn now_utc_plus_sec_str(sec: f64) -> Result<String, Error> {
    let new_time = now_utc() + Duration::seconds_f64(sec);
    let formatted_time = utc_time_to_str(new_time).map_err(|_| Error::TimeError)?;

    Ok(formatted_time)
}

pub fn parse_utc(moment: &str) -> Result<OffsetDateTime, Error> {
    let parsed = OffsetDateTime::parse(moment, &Rfc3339).map_err(|_| Error::TimeError)?;

    Ok(parsed)
}
