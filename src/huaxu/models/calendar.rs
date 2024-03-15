use std::ops::Add;
use chrono::{DateTime, Duration, FixedOffset, Local, TimeDelta, Utc};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Calendar {
    pub status: String,
    pub data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub entries: Vec<Entry>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Entry {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub icon: String,
    pub banner: String,
    pub items: Vec<Item>,
    pub start_time: String,
    pub end_time: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    #[serde(rename = "type")]
    pub type_field: String,
    pub id: i64,
    pub name: String,
    pub quality: Option<i64>,
    pub icon: String,
    pub icon_big: String,
}

impl Entry {
    pub fn get_banner_link(&self) -> String {
        format!("https://assets.huaxu.app/cur/{}.png", self.banner)
    }

    pub fn time_left(&self) -> i64 {
        let current_time = Local::now().to_utc();
        let event  = DateTime::parse_from_rfc3339(&self.end_time).unwrap().to_utc();

        let time_left = current_time - event;
        time_left.num_days() * -1
    }

    pub fn time_passed(&self) -> i64 {
        let current_time = Local::now().to_utc();
        let event  = DateTime::parse_from_rfc3339(&self.start_time).unwrap().to_utc();

        let time_left = current_time - event;
        time_left.num_days() * -1 + 3
    }

    // pub fn get_start_days_within_range_5(&self) -> i64 {
    //     let start_time = DateTime::parse_from_rfc3339(&self.start_time).unwrap().to_utc();
    //     let time_before_the_week_end = Local::now().to_utc() + TimeDelta::try_days(3);


    // }
}