use std::path::Path;

use serde::de::DeserializeOwned;

use crate::{cli::Args, constants, error::MyError, pipeline::cache::Cache};

mod guard;
mod download;
mod cache;

pub fn run_pipeline(args: &Args) -> Result<(), MyError> {
    if !guard::ensure_dependencies() {
        return Err(MyError::Application("Missing dependencies".to_string()));
    }

    let name = &args.name;
    let entry_path = format!("{}/{}", constants::ASSETS_PATH, name);
    let cache_path = format!("{entry_path}/metadata.json");
    let mut cache = Cache::new(cache_path)?;

    if !Path::new(&format!("{entry_path}/{}.mp4", name)).exists() {
        let Some(url) = &args.url else {
            return Err(MyError::Application("Video not found, please specify a url to download it".to_string()));
        };

        let height = args.height.unwrap_or(cached_or_default(&cache, "height", constants::DEFAULT_HEIGHT));
        let fps = args.fps.unwrap_or(cached_or_default(&cache, "fps", constants::DEFAULT_FPS));

        download::download_video(name, url, height, fps)?;
        cache.set("height", height.into())?;
        cache.set("fps", fps.into())?;
    }

    if !Path::new(&format!("{entry_path}/{}.mp3", args.name)).exists() {
        let Some(url) = &args.url else {
            return Err(MyError::Application("Video not found, please specify a url to download it".to_string()));
        };

        download::download_audio(&args.name, url)?;
    }

    Ok(())
}

fn cached_or_default<T>(cache: &Cache, key: &str, default: T) -> T
where
    T: DeserializeOwned + Clone,
{
    cache
        .get(key)
        .and_then(|v| serde_json::from_value(v.clone()).ok())
        .unwrap_or(default)
}