use clap::Parser;

use crate::constants;

#[derive(Parser)]
#[command(about = "ASCII Video Player", long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub url: Option<String>,

    #[arg(short, long, default_value = "default")]
    pub name: String,

    #[arg(long)]
    pub height: Option<usize>,

    #[arg(long)]
    pub fps: Option<u32>
}