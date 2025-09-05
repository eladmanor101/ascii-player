use std::process::Command;

/// Returns true if all required dependencies exist
pub fn ensure_dependencies() -> bool {
    let has_ytdlp = Command::new("yt-dlp").arg("--version").output().is_ok();
    let has_ffmpeg = Command::new("ffmpeg").arg("-version").output().is_ok();

    if !has_ytdlp {
        println!("Dependency yt-dlp not found.");
    }
    if !has_ffmpeg {
        println!("Dependency ffmpeg not found.");
    }

    has_ytdlp && has_ffmpeg
}