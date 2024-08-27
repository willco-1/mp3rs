use reqwest;
use serde::Deserialize;
use crate::config::Config;

#[derive(Deserialize, Debug)]
pub struct Recording {
   pub title: String,
   pub artist_credit: Vec<ArtistCredit>,
}

#[derive(Deserialize, Debug)]
pub struct ArtistCredit {
   pub name: &str,
}

pub async fn get_recording_metadata(title: &str, config: &Config) -> Result<Recording, reqwest::Error> {
    let client = reqwest::Client::new();
    let url = format!("{}/recording/?query={}", config.api_root, title);
    let response = client.get(&url)
        .header("User-Agent", "my_music_cli/0.1.0")
        .send()
        .await?;

    let recordings: Vec<Recording> = response.json().await?;

    Ok(recordings.into_iter().next().unwrap())
}
