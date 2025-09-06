use std::io::{self, Read, BufReader};
use std::process::{self, Command, Stdio};

pub struct Frames {
    reader: BufReader<process::ChildStdout>,
    width: usize,
    height: usize,
}

impl Frames {
    pub fn new(video_path: &str, fps: u32, width: i32, height: i32) -> io::Result<Self> {
        let mut cmd = Command::new("ffmpeg");
        cmd.args([
            "-loglevel", "error",
            "-i", video_path,
            "-vf",
            &format!("fps={fps},scale={width}:{height}:flags=lanczos,crop=iw:ih/2,format=gray"),
            "-f", "rawvideo",
            "-pix_fmt", "gray",
            "-",
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::null());

        let child = cmd.spawn()?;
        let stdout = child.stdout.ok_or(io::Error::new(io::ErrorKind::Other, "no stdout"))?;
        let reader = BufReader::new(stdout);

        let (real_width, real_height) = probe_dimensions(video_path)?;

        Ok(Self {
            reader,
            width: real_width,
            height: real_height
        })
    }

    pub fn next_frame(&mut self) -> io::Result<Option<Vec<u8>>> {
        let frame_size = self.width * self.height;
        let mut buf = vec![0u8; frame_size];
        let mut read = 0;
        while read < frame_size {
            let n = self.reader.read(&mut buf[read..])?;
            if n == 0 {
                // EOF
                return Ok(None);
            }
            read += n;
        }
        Ok(Some(buf))
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}

fn probe_dimensions(video: &str) -> io::Result<(usize, usize)> {
    let output = Command::new("ffprobe")
        .args([
            "-v", "error",
            "-select_streams", "v:0",
            "-show_entries", "stream=width,height",
            "-of", "csv=s=,:p=0",
            video
        ])
        .output()?;

    if !output.status.success() {
        return Err(io::Error::new(io::ErrorKind::Other, format!("ffprobe failed:\n{}", String::from_utf8_lossy(&output.stderr))));
    }

    let out = String::from_utf8_lossy(&output.stdout);
    let parts: Vec<&str> = out.trim().split(',').collect();
    if parts.len() != 2 {
        return Err(io::Error::new(io::ErrorKind::Other, "unexpected ffprobe output"));
    }

    let w: usize = parts[0].parse().unwrap_or(0);
    let h: usize = parts[1].parse().unwrap_or(0);
    Ok((w, h))
}