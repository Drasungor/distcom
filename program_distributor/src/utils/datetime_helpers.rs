use chrono::{NaiveDateTime, NaiveDate, NaiveTime};
use chrono::prelude::*;use std::time::{SystemTime, UNIX_EPOCH};


pub fn get_current_naive_datetime() -> NaiveDateTime {
    let current_system_time = SystemTime::now();
    let since_the_epoch = current_system_time.duration_since(UNIX_EPOCH).expect("Time went backwards");
    let current_datetime = DateTime::from_timestamp_millis(since_the_epoch.as_millis().try_into().unwrap()).unwrap();
    let now_naive_datetime = current_datetime.naive_utc();
    return now_naive_datetime;
}
