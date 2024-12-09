use std::collections::HashMap;

use reqwest::Response;

const YOUTUBE_API_KEY: &str = "";
const CHANNEL_ID: &str = "";
const YOUTUBE_API_URL: &str = "https://www.googleapis.com/youtube/v3";

#[tokio::main]
async fn main() {
    let youtube_channel_url = format!(
        "{}/channels?part=snippet&id={}&key={}",
        YOUTUBE_API_URL, CHANNEL_ID, YOUTUBE_API_KEY,
    );

    let response = reqwest::get(&youtube_channel_url)
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    println!("{:?}", response);
}

fn youtube_channel() {}
