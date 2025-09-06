use std::fs;
use std::process::Command;
use image::{GenericImageView, Luma};

use crate::error::MyError;

pub struct Frames {
    backend: FrameBackend,
    index: usize,
    height: usize,
    width: usize
}

enum FrameBackend {
    PngFiles(Vec<String>), // for now
    // Later: Stream(ffmpeg child process, BufReader, etc.)
}

impl Frames {
    pub fn from_video(video_path: &str, fps: u32, width: i32, height: i32, out_dir: &str) -> Result<Self, MyError> {
        let filter = format!("fps={fps},scale={width}:{height}:flags=lanczos,format=gray");

        fs::create_dir_all(out_dir)?;

        let status = Command::new("ffmpeg")
            .args([
                "-loglevel", "error",
                "-i", video_path,
                "-vf", &filter,
                &format!("{}/frame_%05d.png", out_dir),
            ])
            .status()?;

        if !status.success() {
            return Err(MyError::Player("ffmpeg failed to extract frames".to_string()));
        }

        let mut files: Vec<_> = fs::read_dir(out_dir)?
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let path = entry.path();
                if path.extension()?.to_str()? == "png" {
                    Some(path.to_string_lossy().to_string())
                } else {
                    None
                }
            })
            .collect();

        files.sort();

        let (width, height) = image::open(format!("{}/frame_00001.png", out_dir)).unwrap().dimensions();

        Ok(Self {
            backend: FrameBackend::PngFiles(files),
            index: 0,
            width: width as usize,
            height: height as usize
        })
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}

impl Iterator for Frames {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        match &self.backend {
            FrameBackend::PngFiles(files) => {
                if self.index >= files.len() {
                    return None;
                }

                let file = &files[self.index];
                self.index += 1;

                match image::open(file) {
                    Ok(img) => Some(img.to_luma8().pixels().map(|Luma([l])| *l).collect()),
                    Err(_) => None
                }
            }
            // FrameBackend::Stream => { ... }
        }
    }
}


// use std::io::{self, Read, BufReader};
// use std::process::{self, Command, Stdio};

// use crate::error::MyError;

// pub struct Frames {
//     reader: BufReader<process::ChildStdout>,
//     width: usize,
//     height: usize,
// }

// impl Frames {
//     pub fn new(video_path: &str, fps: u32, width: i32, height: i32) -> Result<Self, MyError> {
//         let mut cmd = Command::new("ffmpeg");
//         cmd.args([
//             "-loglevel", "error",
//             "-i", video_path,
//             "-vf",
//             &format!("fps={fps},scale={width}:{height}:flags=lanczos,crop=iw:ih/2,format=gray"),
//             "-f", "rawvideo",
//             "-pix_fmt", "gray",
//             "-",
//         ])
//         .stdout(Stdio::piped())
//         .stderr(Stdio::null());

//         let child = cmd.spawn()?;
//         let stdout = child.stdout.ok_or(io::Error::new(io::ErrorKind::Other, "no stdout"))?;
//         let reader = BufReader::new(stdout);

//         let (real_width, real_height) = probe_dimensions(video_path)?;

//         Ok(Self {
//             reader,
//             width: real_width,
//             height: real_height
//         })
//     }

//     pub fn next_frame(&mut self) -> Result<Option<Vec<u8>>, MyError> {
//         let frame_size = self.width * self.height;
//         let mut buf = vec![0u8; frame_size];
//         let mut read = 0;
//         while read < frame_size {
//             let n = self.reader.read(&mut buf[read..])?;
//             if n == 0 {
//                 // EOF
//                 return Ok(None);
//             }
//             read += n;
//         }
//         Ok(Some(buf))
//     }

//     pub fn width(&self) -> usize {
//         self.width
//     }

//     pub fn height(&self) -> usize {
//         self.height
//     }
// }

// fn probe_dimensions(video: &str) -> Result<(usize, usize), MyError> {
//     let output = Command::new("ffprobe")
//         .args([
//             "-v", "error",
//             "-select_streams", "v:0",
//             "-show_entries", "stream=width,height",
//             "-of", "csv=s=,:p=0",
//             video
//         ])
//         .output()?;

//     if !output.status.success() {
//         return Err(MyError::Player(format!("ffprobe failed:\n{}", String::from_utf8_lossy(&output.stderr))));
//     }

//     let out = String::from_utf8_lossy(&output.stdout);
//     let parts: Vec<&str> = out.trim().split(',').collect();
//     if parts.len() != 2 {
//         return Err(MyError::Player("unexpected ffprobe output".to_string()));
//     }

//     let w: usize = parts[0].parse().unwrap_or(0);
//     let h: usize = parts[1].parse().unwrap_or(0);
//     Ok((w, h))
// }