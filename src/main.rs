pub mod cli;
pub mod pipeline;
pub mod player;
pub mod common;

use std::{io::{self, Write}, thread, time::{Duration, Instant}};

use clap::Parser;
use crossterm::{ExecutableCommand, cursor, terminal};

use crate::{cli::Args, common::error::MyError, pipeline::run_pipeline, player::frames::Frames};
pub use crate::common::{utils, constants, error};

/*
TODO:
- Sanitize name argument
*/

fn main() -> Result<(), MyError> {
    println!("Rust binary started with args: {:?}", std::env::args().collect::<Vec<_>>());
    let args = Args::parse();

    run_pipeline(&args)?;

    let path = format!("{}/{}", constants::ASSETS_PATH, args.name);
    let video_path = format!("{}/{}.mp4", path, args.name);
    let frames_path = format!("{}/frames", path);
    let mut frames = Frames::from_video(&video_path, 30, -1, constants::DEFAULT_HEIGHT as i32, &frames_path)?;

    let (width, height) = (frames.width(), frames.height());

    let mut stdout = std::io::stdout();
    stdout.execute(cursor::Hide)?;
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;
    stdout.execute(terminal::SetSize(width as u16, height as u16))?;

    let start = Instant::now();
    for (frame_index, frame) in frames.enumerate() {
        player::render::display_frame(height, width, &mut stdout, frame)?;

        let next_frame = start + Duration::from_millis(frame_index as u64 * 1000 / 30.0 as u64);
        let now = Instant::now();
        if next_frame > now {
            thread::sleep(next_frame - now);
        }
    }

    stdout.flush()?;

    Ok(())
}


/*
use std::{env, fs::{self, File}, io::{self, Stdout, Write}, thread, time::{Duration, Instant}};

use crossterm::{
    ExecutableCommand, QueueableCommand,
    cursor, terminal,
    style::{self}
};
use image::{DynamicImage, GenericImageView};

const ASCII_CHARS: [char; 10] = ['@', '%', '#', '*', '+', '=', '-', ':', '.', ' '];

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        return Err(io::Error::new(io::ErrorKind::Other, "Please specify height, width, fps"));
    }

    let height: usize = args[1].parse().unwrap();
    let width: usize = args[2].parse().unwrap();
    let fps: usize = args[3].parse().unwrap();

    let frames_dir = "assets/frames";
    let frame_count = fs::read_dir(frames_dir)?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "png"))
        .count();

    let mut stdout = std::io::stdout();
    stdout.execute(cursor::Hide)?;
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;

    let stream_handle = rodio::OutputStreamBuilder::open_default_stream().unwrap();
    let sink = rodio::Sink::connect_new(stream_handle.mixer());
    let music_file = File::open("assets/badapple.mp3").unwrap();
    sink.set_volume(0.025);
    sink.append(rodio::Decoder::try_from(music_file).unwrap());

    let start = Instant::now();
    for frame_index in 1..frame_count {
        let frame_path = format!("{}/frame_{}.png", frames_dir, frame_index);
        let frame = image::open(frame_path).unwrap();

        display_frame(height, width, &mut stdout, frame)?;

        let next_frame = start + Duration::from_millis(frame_index as u64 * 1000 / fps as u64);
        let now = Instant::now();
        if next_frame > now {
            thread::sleep(next_frame - now);
        }
    }

    stdout.flush()?;

    Ok(())
}

fn display_frame(height: usize, width: usize, stdout: &mut Stdout, frame: DynamicImage) -> io::Result<()> {
    for i in 0..height {
        for j in 0..width {
            let pixel = frame.get_pixel(j as u32, i as u32);
            let brightness = (0.2126 * pixel[0] as f32 + 0.7152 * pixel[1] as f32 + 0.0722 * pixel[2] as f32) / 255.0;
            let ascii_index = ((ASCII_CHARS.len() - 1) as f32 * brightness).round() as usize;
            
            stdout
                .queue(cursor::MoveTo(j as u16, i as u16))?
                .queue(style::Print(ASCII_CHARS[ascii_index]))?;
        }
    }

    Ok(())
}
*/