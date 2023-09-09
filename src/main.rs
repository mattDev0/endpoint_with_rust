use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::to_string_pretty;
use warp::{http::StatusCode, Filter};

#[derive(Debug, Deserialize)]
struct QueryParams {
    track: Option<String>,
    slack_name: Option<String>,
}

#[derive(Debug, Serialize)]
struct Response {
    slack_name: String,
    current_day: String,
    utc_time: String,
    track: String,
    github_file_url: String,
    github_repo_url: String,
    status_code: u16,
}

#[tokio::main]
async fn main() {
    let api = warp::path!("info")
        .and(warp::get())
        .and(warp::query::<QueryParams>())
        .map(|params: QueryParams| {
            let slack_name = params.slack_name.unwrap_or_else(|| "Unknown".to_string());
            let current_day = Utc::now().format("%A").to_string();
            let utc_time = Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string();
            let track = params.track.unwrap_or_else(|| "Unknown".to_string());
            let github_repo_url = "https://github.com/mattDev0/endpoint_with_rust".to_string();
            let github_file_url = format!("{}/blob/master/src/main.rs", github_repo_url);
            let status_code = StatusCode::OK.as_u16();
            let response = Response {
                slack_name,
                current_day,
                utc_time,
                track,
                github_file_url,
                github_repo_url,
                status_code,
            };
            let pretty_json = to_string_pretty(&response).unwrap();
            warp::reply::with_status(pretty_json, StatusCode::OK)
        });

    warp::serve(api).run(([127, 0, 0, 1], 8080)).await;
}