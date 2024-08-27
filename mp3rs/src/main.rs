use std::io;
use walkdir::WalkDir;
use tokio;
use std::fs;
mod config;
mod api;
use clap::Parser;
use api::{Recording, get_recording_metadata};
use config::Args;

async fn rename_mp3_files(directory: &str) -> io::Result<()> {
    let config = config::load_config();

    for entry in WalkDir::new(directory).into_iter().filter_map(Result::ok) {
        if let Some(extension) = entry.path().extension() {
            if extension == "mp3" {
                let path = entry.path().display().to_string();
                let file_name = path.split('/').last().unwrap().split('.').next().unwrap();

                match get_recording_metadata(file_name, &config).await {
                    Ok(recording) => {
                        let artist = recording.artist_credit.first().map(|a| a.name.as_str()).unwrap_or("Unknown");
                        let new_name = format!("{}_{}.mp3", artist, recording.title);
                        let new_path = entry.path().with_file_name(new_name);
                        fs::rename(entry.path(), &new_path)?;
                        println!("Renamed {} to {}", path, new_path.display());
                    },
                    Err(e) => println!("Failed to fetch metadata for {}: {}", file_name, e),
                }
            }
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let args = Args::parse();
    rename_mp3_files(&args.dir).await?;
    Ok(())
}

