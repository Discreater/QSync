use crate::google::protobuf::Timestamp;
use chrono::{DateTime, NaiveDateTime, Utc};

pub fn convert_to_timestamp(dt: &DateTime<Utc>) -> Timestamp {
  Timestamp {
    seconds: dt.timestamp(),
    nanos: dt.timestamp_subsec_nanos() as i32,
  }
}

pub fn convert_from_timestamp(t: &Timestamp) -> Option<DateTime<Utc>> {
  let naive_dt = NaiveDateTime::from_timestamp_opt(t.seconds, t.nanos as u32);
  naive_dt.map(|t| DateTime::<Utc>::from_naive_utc_and_offset(t, Utc))
}
