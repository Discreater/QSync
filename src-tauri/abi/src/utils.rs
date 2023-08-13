use chrono::{DateTime, Utc};
use prost_types::Timestamp;

pub fn convert_to_timestamp(dt: &DateTime<Utc>) -> Timestamp {
  Timestamp {
    seconds: dt.timestamp(),
    nanos: dt.timestamp_subsec_nanos() as i32,
  }
}
