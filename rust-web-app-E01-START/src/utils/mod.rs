mod error;
pub use error::{Error, Result};

use time::format_description::well_known::Rfc3339;
use time::{Duration, OffsetDateTime};

pub fn now_utc() -> OffsetDateTime {
    OffsetDateTime::now_utc()
}

pub fn format_time(time: OffsetDateTime) -> String {
    time.format(&Rfc3339).unwrap() // todo: need to check if its safe
}

pub fn now_utc_plus_sec_str(sec: f64) -> String {
    let new_time = now_utc() + Duration::seconds_f64(sec);
    format_time(new_time)
}

pub fn parse_utc(moment: &str) -> Result<OffsetDateTime> {
    OffsetDateTime::parse(moment, &Rfc3339)
    .map_err(|_| Error::DateFailParse(moment.to_string()))
}

pub fn encode_b64u(content: &str) -> String {
    base64_url::encode(content)
}

pub fn decode_b64u(content: &str) -> Result<String> {
    let decoded_string = base64_url::decode(content)
    .ok()
    .and_then(|r| String::from_utf8(r).ok())
    .ok_or(Error::FaildedToB64uDecode)?;

    Ok(decoded_string)
}