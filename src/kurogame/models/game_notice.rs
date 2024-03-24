use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameNotice {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "ModifyTime")]
    pub modify_time: i64,
    #[serde(rename = "Content")]
    pub content: Vec<Content>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content {
    #[serde(rename = "Id")]
    pub id: i64,
    #[serde(rename = "PicAddr")]
    pub pic_addr: String,
    #[serde(rename = "JumpType")]
    pub jump_type: String,
    #[serde(rename = "JumpAddr")]
    pub jump_addr: String,
    #[serde(rename = "PicType")]
    pub pic_type: String,
    #[serde(rename = "Interval")]
    pub interval: i64,
    #[serde(rename = "BeginTime")]
    pub begin_time: i64,
    #[serde(rename = "EndTime")]
    pub end_time: i64,
    #[serde(rename = "AppearanceDay")]
    pub appearance_day: Vec<Value>,
    #[serde(rename = "AppearanceCondition")]
    pub appearance_condition: Vec<Value>,
    #[serde(rename = "DisappearanceCondition")]
    pub disappearance_condition: Vec<Value>,
    #[serde(rename = "AppearanceTime")]
    pub appearance_time: Vec<Value>,
}
