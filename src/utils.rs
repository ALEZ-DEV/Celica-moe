use chrono::{DateTime, Local, TimeZone, Utc};

pub fn get_utc_0_current_time() -> DateTime<Utc> {
    chrono_tz::Europe::London
        .from_utc_datetime(&Local::now().naive_local())
        .to_utc()
}
