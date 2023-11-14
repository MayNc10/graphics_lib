use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long)]
    pub target_fps: Option<u64>,
    #[arg(short, long)]
    pub aspect_ratio: Option<f32>,
    #[arg(short, long)]
    pub image_width: Option<i32>,
    #[arg(long)]
    pub vfov: Option<f32>,
    #[arg(short, long)]
    pub samples_per_pixel: Option<i32>,
    #[arg(short, long)]
    pub max_depth: Option<i32>,
    #[arg(short, long)]
    pub no_parallel: bool,
    #[arg(short, long)]
    pub verbose: bool,
}

