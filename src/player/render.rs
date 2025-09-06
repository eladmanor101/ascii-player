use std::io::{self, Stdout};

use crossterm::{QueueableCommand, cursor, style};

use crate::{constants, error::MyError};

pub fn display_frame(height: usize, width: usize, stdout: &mut Stdout, frame: Vec<u8>) -> Result<(), MyError> {
    for i in 0..height {
        for j in 0..width {
            let brightness = frame[i * height + j] as f32 / 255.0;
            let ascii_index = ((constants::ASCII_CHARS.len() - 1) as f32 * brightness).round() as usize;
            
            stdout
                .queue(cursor::MoveTo(j as u16, i as u16))?
                .queue(style::Print(constants::ASCII_CHARS[ascii_index]))?;
        }
    }

    Ok(())
}