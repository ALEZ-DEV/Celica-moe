use super::models::game_notice::GameNotice;

static KUROGAME_BASE_URL: &str = "http://prod-encdn-volcdn.kurogame.net/prod/";

fn build_url(uri: &str) -> String {
    format!("{}{}", KUROGAME_BASE_URL, uri)
}

pub async fn fetch_notice() -> anyhow::Result<GameNotice> {
    let uri = "client/notice/config/com.kurogame.punishing.grayraven.en/2.1.0/ScrollPicNotice.json";
    let response = reqwest::get(build_url(uri)).await?;

    let notice: GameNotice = serde_json::from_str(&response.text().await?)?;

    Ok(notice)
}
