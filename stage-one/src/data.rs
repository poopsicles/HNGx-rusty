use axum::http::StatusCode;
use serde::{Serializer, Serialize, ser::Error};
use time::{format_description::well_known::Rfc3339, OffsetDateTime, Weekday};

use crate::ApiParams;

#[derive(Debug, Serialize)]
pub struct Data {
    pub slack_name: String,
    #[serde(serialize_with = "weekday_serialize")]
    pub current_day: Weekday,
    #[serde(serialize_with = "datetime_serialize")]
    pub utc_time: OffsetDateTime,
    pub track: String,
    pub github_file_url: String,
    pub github_repo_url: String,
    #[serde(serialize_with = "statuscode_serialize")]
    pub status_code: StatusCode,
}

fn statuscode_serialize<S>(x: &StatusCode, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_u16(x.as_u16())
}

fn datetime_serialize<S>(x: &OffsetDateTime, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match x.format(&Rfc3339) {
        Ok(x) => s.serialize_str(&x),
        Err(_) => Err(S::Error::custom("failed formatting `OffsetDateTime`")),
    }
}

fn weekday_serialize<S>(x: &Weekday, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    x.to_string().serialize(s)
}

impl Default for Data {
    fn default() -> Self {
        Self {
            slack_name: String::new(),
            current_day: OffsetDateTime::now_utc().weekday(),
            utc_time: OffsetDateTime::now_utc()
                .replace_nanosecond(0)
                .expect("could not strip nanoseconds"),
            track: String::new(),
            github_file_url: String::from(""),
            github_repo_url: String::from(""),
            status_code: StatusCode::OK,
        }
    }
}

impl From<ApiParams> for Data {
    fn from(value: ApiParams) -> Self {
        Self {
            slack_name: value.slack_name,
            track: value.track,
            ..Default::default()
        }
    }
}
