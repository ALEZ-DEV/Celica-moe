use crate::huaxu::models::item::Item;
use crate::kurogame::api::fetch_notice;
use chrono::DateTime;
use chrono::TimeZone;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Banners {
    pub status: String,
    pub data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub groups: Vec<Group>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Group {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub background: String,
    pub banners: Vec<Banner>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Banner {
    pub id: i64,
    pub name: String,
    pub cost: Cost,
    pub pity: i64,
    pub targets: Vec<Target>,
    pub rules: Vec<String>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    #[serde(skip)]
    pub main_banner: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cost {
    #[serde(rename = "type")]
    pub type_field: String,
    pub id: i64,
    pub name: String,
    pub description: String,
    pub count: i64,
    pub quality: i64,
    pub icon: String,
    pub icon_big: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Target {
    pub id: i64,
    pub items: Vec<Item>,
    pub description: String,
}

impl Banner {
    pub fn get_start_time(&self) -> DateTime<Utc> {
        match &self.start_time {
            Some(t) => DateTime::parse_from_rfc3339(t).unwrap().to_utc(),
            None => Utc::now(),
        }
    }

    pub fn get_end_time(&self) -> DateTime<Utc> {
        match &self.end_time {
            Some(t) => DateTime::parse_from_rfc3339(t).unwrap().to_utc(),
            None => Utc::now(),
        }
    }

    pub async fn fetch_main_banner(&mut self) -> anyhow::Result<()> {
        if self.start_time.is_some() && self.start_time.is_some() {
            let notice = fetch_notice().await?;

            let start_time = self.get_start_time();
            let end_time = self.get_end_time();

            let banner = notice
                .content
                .iter()
                .find(|x| {
                    Utc.timestamp_opt(x.begin_time, 0).unwrap() == start_time
                        && Utc.timestamp_opt(x.end_time, 0).unwrap() == end_time
                })
                .unwrap()
                .pic_addr
                .clone();

            self.main_banner = Some(banner);
        }

        Ok(())
    }
}
