use crate::huaxu::models::{banners::Banners, calendar::Calendar};

static HUAXU_BASE_URL: &str = "https://api.huaxu.app/";

fn build_url(uri: &str) -> String {
    format!("{}{}", HUAXU_BASE_URL, uri)
}

pub async fn fetch_calendar() -> anyhow::Result<Calendar> {
    let response = reqwest::get(build_url("calendar")).await?;

    let calendar: Calendar = serde_json::from_str(&response.text().await?)?;

    Ok(calendar)
}

pub async fn fetch_banner() -> anyhow::Result<Banners> {
    let response = reqwest::get(build_url("banners")).await?;

    let banners: Banners = serde_json::from_str(&response.text().await?)?;

    Ok(banners)
}
