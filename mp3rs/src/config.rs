use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use clap::Parser;
#[derive(Deserialize, Debug)]
pub struct Config {
    pub api_root: String,
    pub api_key: Option<String>,
}
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Directory containing MP3 files
    #[arg(short, long)]
    pub dir: String,
}

pub fn load_config() -> Config {
    let mut file = File::open("config.json").expect("config.json not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read config.json");
    serde_json::from_str(&contents).expect("Failed to parse config.json")
}
