//! This module will be removed soon

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
/// A struct representing command-line arguments
pub struct Cli {
    #[arg(short, long)]
    /// The given target FPS
    pub target_fps: Option<u64>,
    /// The given aspect ratio
    #[arg(short, long)]
    pub aspect_ratio: Option<f32>,
    /// The given image width
    #[arg(short, long)]
    pub image_width: Option<i32>,
    /// The given vertical FOV
    #[arg(long)]
    pub vfov: Option<f32>,
    /// The given samples per pixel
    #[arg(short, long)]
    pub samples_per_pixel: Option<i32>,
    /// The given maximum depth
    #[arg(short, long)]
    pub max_depth: Option<i32>,
    /// Whether the engine should render using multiple threads
    #[arg(short, long)]
    pub no_parallel: bool,
    /// Whether the engine should print debug output
    #[arg(short, long)]
    pub verbose: bool,
}

