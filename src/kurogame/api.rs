use http::{Error, HeaderName, HeaderValue};
use leptos::{
    expect_context, leptos_dom::logging::console_log, server, ErrorBoundary, ServerFnError,
};

use crate::error_template::AppError;

use super::models::game_notice::GameNotice;

static KUROGAME_BASE_URL: &str = "http://prod-encdn-volcdn.kurogame.net/prod/";

#[cfg(debug_assertions)]
static BASE_URL: &str = "localhost:3000/";

#[cfg(not(debug_assertions))]
static BASE_URL: &str = "celica.moe/";

fn build_client_url(uri: &str) -> String {
    format!("{}{}", BASE_URL, uri)
}

fn build_server_url(uri: &str) -> String {
    format!("{}{}", KUROGAME_BASE_URL, uri)
}

pub async fn fetch_notice() -> anyhow::Result<GameNotice> {
    let response = fetch_notice_without_cors().await;

    console_log(&format!("response : {:?}", response));

    if let Ok(r) = response {
        let notice: GameNotice = serde_json::from_str(&r)?;

        return Ok(notice);
    }

    Err(AppError::NotFound.into())
}

#[server(FetchNotice, "/api/kuro")]
pub async fn fetch_notice_without_cors() -> Result<String, ServerFnError> {
    use axum::http::header;
    use leptos_axum::ResponseOptions;

    let response_to_return: leptos_axum::ResponseOptions = expect_context();

    let uri = "client/notice/config/com.kurogame.punishing.grayraven.en/2.1.0/ScrollPicNotice.json";
    let url = build_server_url(uri);
    console_log(&format!("will fetch : {}", url));
    let response = reqwest::get(url).await?;

    response_to_return.insert_header(
        HeaderName::from_lowercase(b"access-control-allow-origin").unwrap(),
        HeaderValue::from_str("*").unwrap(),
    );

    let notice_json = response.text().await.unwrap();

    Ok(notice_json)
}
