use std::process::Command;

use crate::constants;

pub fn download_video(name: &str, yt_link: &str) {
    Command::new("yt-dlp")
        .args([
            "-f", "bestvideo[ext=mp4]/bestvideo",
            "-o", &format!("{}/{}.mp4", constants::ASSETS_PATH, name),
            yt_link
        ])
        .output()
        .expect("yt-dlp video download failed");
}

pub fn download_audio(name: &str, yt_link: &str) {
    Command::new("yt-dlp")
        .args([
            "-f", "bestaudio",
            "--extract-audio",
            "--audio-format", "mp3",
            "-o", &format!("{}/{}.mp3", constants::ASSETS_PATH, name),
            yt_link
        ])
        .output()
        .expect("yt-dlp audio download failed");
}