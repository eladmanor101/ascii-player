use std::{io, path::Path};

use crate::{cli::Args, constants, pipeline::cache::Cache, util::err};

mod guard;
mod download;
mod cache;

pub fn run_pipeline(args: Args) -> io::Result<()> {
    if !guard::ensure_dependencies() {
        return err("Missing dependencies");
    }

    let entry_path = format!("{}/{}", constants::ASSETS_PATH, args.name);
    let cache_path = format!("{entry_path}/metadata.json");
    let mut cache = Cache::new(cache_path)?;

    if !Path::new(&format!("{entry_path}/{}.mp4", args.name)).exists() {
        let Some(url) = &args.url else {
            return err("Video not found, please specify a url to download it");
        };

        download::download_video(&args.name, url);
    }

    if !Path::new(&format!("{entry_path}/{}.mp3", args.name)).exists() {
        let Some(url) = &args.url else {
            return err("Video not found, please specify a url to download it");
        };

        download::download_audio(&args.name, url);
    }

    Ok(())
}