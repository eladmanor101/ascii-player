use std::{io::Write, process::Command};

use crate::{constants, error::MyError};

pub fn download_video(name: &str, yt_link: &str, height: usize, fps: u32) -> Result<(), MyError> {
    let output = Command::new("yt-dlp")
        .args([
            "-f", "bestvideo[ext=mp4]/bestvideo",
            "-o", &format!("{}/{}/{}.mp4", constants::ASSETS_PATH, name, name),
            "--recode-video", "mp4",
            "--postprocessor-args", &format!("-vf scale=-2:{}:flags=lanczos,fps={}", height, fps),
            yt_link
        ])
        .output()?;

    if !output.status.success() {
        return Err(MyError::Pipeline(format!("Video download failed: {:?}", String::from_utf8_lossy(&output.stderr))))
    }

    Ok(())
}

pub fn download_audio(name: &str, yt_link: &str) -> Result<(), MyError> {
    let output = Command::new("yt-dlp")
        .args([
            "-f", "bestaudio",
            "--extract-audio",
            "--audio-format", "mp3",
            "-o", &format!("{}/{}/{}.mp3", constants::ASSETS_PATH, name, name),
            yt_link
        ])
        .output()?;

    if !output.status.success() {
        return Err(MyError::Pipeline(format!("Audio download failed: {:?}", String::from_utf8_lossy(&output.stderr))))
    }

    Ok(())
}