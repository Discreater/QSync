use prost_types::Timestamp;
use sqlx::types::chrono::NaiveDateTime;

pub fn convert_to_timestamp(dt: &NaiveDateTime) -> Timestamp {
  Timestamp {
    seconds: dt.timestamp(),
    nanos: dt.timestamp_subsec_nanos() as i32,
  }
}
