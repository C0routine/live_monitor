use serde::Deserialize;

use crate::constants;

#[tokio::main]
pub async fn search_channel() {
    let youtube_channel_url = format!(
        "{}/search?part=snippet&type=channel&q={}&key={}",
        constants::YOUTUBE_API_URL,
        constants::CHANNEL_NAME,
        constants::YOUTUBE_API_KEY,
    );

    let response = reqwest::get(&youtube_channel_url)
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    println!("{:?}", response);
}

#[derive(Deserialize, Debug)]
pub struct SearchResponse {
    pub items: i32,
}
