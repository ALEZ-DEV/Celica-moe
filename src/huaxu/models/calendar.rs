use std::ops::Add;
use chrono::{Datelike, DateTime, Duration, FixedOffset, Local, TimeDelta, Utc};
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

impl Data {
    pub fn filter_date(&mut self) {
        self.entries = self.clone().entries.into_iter().filter(|e| !e.has_left(3)).filter(|e| e.has_begin(3)).collect();
    }
}

impl Entry {
    pub fn get_banner_link(&self) -> String {
        format!("https://assets.huaxu.app/cur/{}.png", self.banner)
    }

    pub fn get_start_time(&self) -> DateTime<Utc> {
        DateTime::parse_from_rfc3339(&self.start_time).unwrap().to_utc()
    }

    pub fn get_end_time(&self) -> DateTime<Utc> {
        DateTime::parse_from_rfc3339(&self.end_time).unwrap().to_utc()
    }

    pub fn time_passed(&self) -> i64 {
        let current_time = Local::now().to_utc();
        let start_time = self.get_start_time();

        let time_left =  current_time - start_time;

        if start_time.year() == 1970 {
            0
        } else {
            time_left.num_days()
        }
    }

    pub fn time_left(&self) -> i64 {
        let current_time = Local::now().to_utc();
        let end_time = self.get_end_time();

        let time_left = end_time - current_time;

        if end_time.year() == 1970 {
            0
        } else {
            time_left.num_days()
        }
    }

    pub fn has_left(&self, expire_range: i64) -> bool {
        let end_time = Local::now().to_utc() - TimeDelta::try_days(expire_range).unwrap();
        end_time > self.get_end_time()
    }

    pub fn has_begin(&self, begin_range: i64) -> bool {
        let begin_time = Local::now().to_utc() + TimeDelta::try_days(begin_range).unwrap();
        begin_time > self.get_start_time()
    }
}