use crate::huaxu::models::calendar::Calendar;

pub async fn fetch_calendar() -> anyhow::Result<Calendar> {
    let response = reqwest::get("https://api.huaxu.app/calendar").await.unwrap();

    let calendar: Calendar  = serde_json::from_str(&response.text().await.unwrap()).unwrap();

    Ok(calendar)
}