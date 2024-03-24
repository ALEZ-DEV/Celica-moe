use crate::{huaxu::models::item::Item, utils::get_utc_0_current_time};
use chrono::{DateTime, Datelike, Local, TimeDelta, TimeZone, Utc};
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
    #[serde(skip)]
    pub selected: bool,
}

impl Data {
    pub fn filter_date(&mut self) {
        self.entries = self
            .clone()
            .entries
            .into_iter()
            .filter(|e| !e.has_left(3))
            .filter(|e| e.has_begin(3))
            .collect();
    }
}

impl Entry {
    pub fn get_banner_link(&self) -> String {
        format!("https://assets.huaxu.app/cur/{}.png", self.banner)
    }

    pub fn get_start_time(&self) -> DateTime<Utc> {
        DateTime::parse_from_rfc3339(&self.start_time)
            .unwrap()
            .to_utc()
    }

    pub fn get_end_time(&self) -> DateTime<Utc> {
        DateTime::parse_from_rfc3339(&self.end_time)
            .unwrap()
            .to_utc()
    }

    pub fn time_passed(&self) -> i64 {
        let current_time = get_utc_0_current_time();
        let start_time = self.get_start_time();

        let time_left = current_time - start_time;

        if start_time.year() == 1970 {
            0
        } else {
            time_left.num_days()
        }
    }

    pub fn time_left(&self) -> i64 {
        let current_time = get_utc_0_current_time();
        let end_time = self.get_end_time();

        let time_left = end_time - current_time;

        if end_time.year() == 1970 {
            0
        } else {
            time_left.num_days()
        }
    }

    pub fn has_left(&self, expire_range: i64) -> bool {
        let end_time = get_utc_0_current_time() - TimeDelta::try_days(expire_range).unwrap();
        end_time > self.get_end_time()
    }

    pub fn has_few_hour_left(&self) -> bool {
        let current_time = get_utc_0_current_time();
        let time_left = current_time - self.get_end_time();
        time_left.num_days() == 0 && time_left.num_hours() >= 0
    }

    pub fn has_begin(&self, begin_range: i64) -> bool {
        let begin_time = get_utc_0_current_time() + TimeDelta::try_days(begin_range).unwrap();
        begin_time > self.get_start_time()
    }
}

impl Item {
    pub fn get_icon_link(&self) -> String {
        format!("https://assets.huaxu.app/cur/{}.png", self.icon)
    }
}
