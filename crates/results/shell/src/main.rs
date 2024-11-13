#![doc = include_str!("../README.md")]

use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Directory containing the datasets
    #[arg(short('i'), long)]
    data_dir: PathBuf,
    
    /// Path to the output directory.
    #[arg(short('o'), long)]
    out_dir: Option<PathBuf>,

    /// Pre-trained model to use
    #[arg(short('m'), long)]
    model: Option<PathBuf>,
}

fn main() -> Result<(), String> {
    let args = Args::parse();

    let data_dir = args.data_dir.canonicalize().map_err(|e| format!("Could not canonicalize data directory: {}", e))?;
    println!("Reading datasets from: {data_dir:?}");

    let out_dir = args.out_dir.unwrap_or_else(|| data_dir.join("results"));
    if !out_dir.exists() {
        std::fs::create_dir(&out_dir).map_err(|e| format!("Could not create output directory: {}", e))?;
    }
    let out_dir = out_dir.canonicalize().map_err(|e| format!("Could not canonicalize output directory: {}", e))?;
    println!("Saving results to: {out_dir:?}");

    // Check if model has a value
    if let Some(model) = args.model {
        let model = model.canonicalize().map_err(|e| format!("Could not canonicalize model path: {}", e))?;
        println!("Using pre-trained model: {model:?}");
    }
    // Check if model filename exists

    Ok(())
}
