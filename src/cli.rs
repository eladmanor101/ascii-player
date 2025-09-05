use clap::Parser;

#[derive(Parser)]
#[command(about = "ASCII Video Player", long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub url: Option<String>,

    #[arg(short, long, default_value = "default")]
    pub name: String
}